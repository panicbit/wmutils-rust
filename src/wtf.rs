extern crate lax;

use std::env;
use std::process;
use lax::prelude::*;
use lax::window::RevertFocus;

pub mod util;

fn usage(programname: &String) {
    println!("Usage: {} <wid>", programname);
    process::exit(1);
}

fn main() {
    let programname = env::args().nth(0).unwrap_or_else(|| String::new());
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 || args[1] == "-h" || args[1] == "--help" {
        usage(&programname);
    }

    let connection = util::init_lax(&programname);

    let window_id = util::get_window_id(&args[1]);
    let window = WindowRef::from(&connection, window_id);

    window.focus(RevertFocus::PointerRoot);

    connection.flush();
}
