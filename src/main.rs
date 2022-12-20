mod arg_parse;
mod handle;
mod work_opts;

use handle::eval_config;
use std::env;
use std::io;
use std::io::Read;
use work_opts::Config;
use Default;

/**
 *
 *  ls ｜ xbat -e default(\d+).jpg ccx $0 $1 $2
 *
 */
fn main() -> Result<(), i32> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut all_from_pipe = String::new();

    let mut buf = String::new();

    while let Ok(n_bytes) = handle.read_to_string(&mut buf) {
        if n_bytes == 0 {
            break;
        }
        all_from_pipe.push_str(buf.as_str());
        buf.clear();
    }

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
    let exec_ret = eval_config(&all_from_pipe, &conf);

    if 0 == exec_ret {
        Ok(())
    } else {
        Err(exec_ret)
    }
}
