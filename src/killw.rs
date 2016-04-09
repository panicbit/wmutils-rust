extern crate xcb;
extern crate clap;
extern crate lax;

use clap::{App, Arg};
use lax::prelude::*;

pub mod util;

fn main() {
    let args = App::new("killw")
        .about("kill windows")
        .arg(Arg::with_name("parent")
            .short("p")
            .help("Kill the parent application of the window instead of the window itself"))
        .arg(Arg::with_name("wid")
            .multiple(true)
            .required(true))
        .get_matches();

    let connection = util::init_lax("killw");
    // Unwrap is fine, because the `wid` arg is required
    let windows = args.values_of("wid").unwrap()
        .map(|wid| WindowRef::from(&connection, util::get_window_id(&wid)));

    for window in windows {
        if args.is_present("parent") {
            window.kill_client();
        } else {
            window.destroy();
        }
    }

    connection.flush();
}
