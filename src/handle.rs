use crate::work_opts::Config;
use lazy_static::lazy_static;
use nix::{libc::fork, unistd::execvp};
use regex::Regex;
use std::{ffi::CString, process::exit};

pub(crate) fn eval_config(input_need_to_splite: &String, conf: &Config) {
    let mut lines: Box<dyn Iterator<Item = &str>> = if conf.spliter.is_none() {
        Box::new(input_need_to_splite.lines())
    } else {
        Box::new(input_need_to_splite.split(conf.spliter.as_ref().unwrap()))
    };

    if conf.trim_each_line {
        lines = Box::new(lines.filter_map(|x| Some(x.trim())));
    }

    if conf.skip_empty_line {
        lines = Box::new(lines.filter(|x| !x.is_empty()));
    }

    lines.for_each(|x| {
        let capture_from_single_splite = parse_capture(x, conf);
        eval(&capture_from_single_splite, conf);
    });
}

///
///
fn parse_capture<'a>(splite_single_line: &'a str, conf: &'a Config) -> Vec<&'a str> {
    let mut inputs: Vec<&str> = Vec::new();

    if conf.regexp.is_none() {
        inputs.push(splite_single_line);
    } else {
        let reg = conf.regexp.as_ref().unwrap();

        let mut capts_iter = reg.captures_iter(splite_single_line);

        match capts_iter.next() {
            None => {
                if conf.not_math_put_all {
                    inputs.push(splite_single_line);
                }
            }
            Some(capt) => {
                inputs.extend(capt.iter().map(|x| x.unwrap().as_str()));
            }
        }
    }
    inputs
}

fn to_cstr(inn: &String) -> CString {
    CString::new(inn.as_str()).unwrap()
}

fn eval(capture_from_single_splite: &Vec<&str>, conf: &Config) {
    let command_and_args: Vec<String> = conf
        .commands
        .iter()
        .map(|x| fullfill_string(capture_from_single_splite, x, conf))
        .collect();

    if command_and_args.len() <= 0 {
        exit(1);
    }

    let mut it = command_and_args.iter();

    let exec_command = to_cstr(it.next().unwrap());

    let exec_command_on_args = exec_command.clone();

    let mut exec_args: Vec<CString> = Vec::new();
    exec_args.push(exec_command_on_args);

    it.map(|x| to_cstr(x)).for_each(|x| exec_args.push(x));

    let pid = unsafe { fork() };

    if -1 == pid {
        exit(1);
    } else if 0 == pid {
        match execvp(&exec_command, &exec_args) {
            Ok(_x) => {
                print!("ok\n")
            }
            Err(_e) => {
                print!("err\n")
            }
        };
    } else {
    }
}

fn fullfill_string(captuers: &Vec<&str>, orig_str: &str, conf: &Config) -> String {
    let mut left_poss: Vec<(usize, &str)> = orig_str.match_indices(conf.left).rev().collect();

    if left_poss.len() == 0 {
        return String::from(orig_str);
    }

    let mut right_poss: Vec<(usize, &str)> = orig_str.match_indices(conf.right).rev().collect();

    if right_poss.len() == 0 {
        return String::from(orig_str);
    }

    let mut com = String::from("");

    let mut right: Option<usize> = None;

    let mut last = 0;

    while let Some(l_pos) = left_poss.pop() {
        let left = l_pos.0;

        //
        if right.is_none() || right.unwrap() <= left {
            while let Some(r_pos) = right_poss.pop() {
                if r_pos.0 > left {
                    right = Some(r_pos.0);
                    break;
                }
            }
        }

        // 最后一个左匹配没有找到右匹配
        if right.is_none() {
            com.push_str(&orig_str[last..]);
            last = orig_str.len() - 1;
            break;
        }

        let slice = &orig_str[left + 1..right.unwrap()];

        lazy_static! {
            static ref NUMBER_NIC: Regex = Regex::new(r"^\s*(\d*)\s*$").unwrap();
        }

        if NUMBER_NIC.is_match(slice) {
            if last != left {
                com.push_str(&orig_str[last..left]);
                last = left;
            }

            let caps = NUMBER_NIC
                .captures_iter(slice)
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            let uidx = if caps.len() == 0 {
                0
            } else {
                caps.parse::<usize>().unwrap()
            };

            if uidx >= captuers.len() {
                print!("补货长度不对");
                exit(1);
            } else {
                com.push_str(captuers.get(uidx).as_ref().unwrap());
                last = right.unwrap() + 1;
            }
            right = None;
        } else {
            continue;
        }
    }

    if last != orig_str.len() - 1 {
        let ccc = &orig_str[last..orig_str.len()];
        com.push_str(ccc);
    }

    com
}
