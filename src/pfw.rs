// Implementation of wmutils' pfw (print focused window)

extern crate lax;

use std::env;

pub mod util;

fn main() {
    let programname = env::args().nth(0).unwrap_or_else(|| String::new());

    let connection = util::init_lax(&programname);

    match connection.focused_window_ref() {
        Ok(w) => println!("0x{:08x}", w.id()),
        Err(e) => println!("Error: {:?}", e)
    }
}
