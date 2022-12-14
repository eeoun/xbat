use crate::work_opts::Config;
use regex::Regex;
use std::process::{self, exit};

pub(crate) fn parse_lr(opts_and_commands: &Vec<String>, conf: &mut Config) -> bool {
    let mut idx = 0;
    let mut is_on_command = false;

    while idx < opts_and_commands.len() {
        let dd = &opts_and_commands[idx];
        idx += 1;

        // 一旦进入command，所有的都算作command
        if is_on_command {
            conf.commands.push(String::from(dd));
            continue;
        }

        // 接受regexp 输入
        if dd.eq("-e") {
            if idx >= opts_and_commands.len() {
                print!("没有正则表达式")
            }

            let regexp_str = &opts_and_commands[idx];

            if regexp_str.is_empty() {
                print!("正则表达式长度为0")
            }

            // 下一个参数跟着的就是正则表达式
            let re_ret = Regex::new(&opts_and_commands[idx]);

            // 正则表达式错误
            if re_ret.is_err() {
                process::exit(1);
            }
            conf.regexp = Some(re_ret.unwrap());
            idx += 1;
            continue;
        }

        // 接受参数-I
        if dd.starts_with("-I") && dd.len() == 4 && dd.is_ascii() {
            match dd.chars().nth(2) {
                Some(cc) => {
                    conf.left = cc;
                }
                None => todo!("x"),
            }

            match dd.chars().nth(3) {
                Some(cc) => {
                    conf.right = cc;
                }
                None => todo!("x"),
            }

            continue;
        }

        if dd.starts_with('-') {
            if dd.contains("v") {
                conf.verbose = true;
            }
            continue;
        }

        if dd.starts_with('-') {
            print!("不支持的选项 丢弃:{}", dd);
            continue;
        }

        is_on_command = true;
        conf.commands.push(String::from(dd));
    }

    if conf.commands.is_empty() {
        print!("Empty");
        exit(0);
    }

    true
}
