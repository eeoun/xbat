mod arg_parse;
mod handle;
mod work_opts;

use handle::eval_config;
use std::env;
use std::fs;
use std::io;
use work_opts::Config;
use Default;

/**
 *
 *  ls ｜ xbat -e default(\d+).jpg ccx $0 $1 $2
 *
 */
fn main() {
    let mut buffer = String::new();

    buffer = fs::read_to_string("./f.txt").expect("Should have been able to read the file");

    // let mut lines = io::stdin().lines();
    // while let Some(line) = lines.next() {
    //     match line {
    //         Ok(line_str) => {
    //             buffer.push_str(line_str.as_str());
    //         }
    //         _ => {}
    //     }
    // }

    let mut conf: Config = Default::default();

    let mut args = env::args();
    // 去掉第一个程序名称
    args.next();
    let mut opts_and_commands: Vec<String> = Vec::new();
    loop {
        match args.next() {
            Some(str) => opts_and_commands.push(str),
            None => break,
        }
    }
    arg_parse::parse_lr(&opts_and_commands, &mut conf);
    eval_config(&buffer, &conf);
}
