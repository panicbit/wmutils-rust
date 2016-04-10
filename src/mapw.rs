extern crate lax;
extern crate xcb;
extern crate clap;

use clap::{App, Arg, ArgGroup};
use lax::prelude::*;

pub mod util;

fn main() {
    let args = App::new("mapw")
        .about("map or unmap windows")
        .arg(Arg::with_name("map").short("m").help("Map (show) wid."))
        .arg(Arg::with_name("toggle").short("t").help("Toggle wid's visibility."))
        .arg(Arg::with_name("unmap").short("u").help("Unmap (hide) wid."))
        .group(ArgGroup::with_name("options").required(true).args(&[
            "map", "toggle", "unmap"
        ]))
        .arg(Arg::with_name("wid")
            .multiple(true)
            .required(true))
        .get_matches();

    let connection = util::init_lax("mapw");
    let windows = args.values_of("wid").unwrap()
        .map(|wid| WindowRef::from(&connection, util::get_window_id(&wid)));

    let action: fn(_) =
        if args.is_present("map") { WindowRef::map }
        else if args.is_present("unmap") { WindowRef::unmap }
        else { toggle };

    for window in windows {
        action(window);
    }

    connection.flush();
}

fn toggle(window: WindowRef) {
    match window.is_mapped() {
        true  => window.unmap(),
        false => window.map()
    }
}
