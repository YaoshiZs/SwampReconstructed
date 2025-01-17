mod compiler;
mod computer;
mod exec;
mod public;
mod utils;
use std::collections::VecDeque;
use std::{env, io};

use exec::args;
use public::env::Env;

fn main() -> io::Result<()> {
    let mut args: VecDeque<String> = env::args().collect();

    let self_name = args.pop_front().unwrap();
    let calc_env = Env::init(self_name);

    args::entry(args, calc_env)?;
    Ok(())
}
