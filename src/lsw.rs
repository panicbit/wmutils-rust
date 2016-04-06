extern crate xcb;
extern crate clap;
extern crate lax;
mod util;

use clap::{Arg, App};
use lax::prelude::*;
use lax::window::MapState;

#[derive(Copy,Clone)]
struct Flags {
    all   : bool,
    hidden: bool,
    ignore: bool,
}

impl Flags {
    fn none(self) -> bool {
           !self.all
        && !self.hidden
        && !self.ignore
    }
}

fn main() {
    let args = App::new("lsw")
        .about("list child windows")
        .arg(Arg::with_name("a").short("a").help("List all windows."))
        .arg(Arg::with_name("r").short("r").help("Print the ID of the root window."))
        .arg(Arg::with_name("o").short("o").help("List windows whose override_redirect attribute is set to 1."))
        .arg(Arg::with_name("u").short("u").help("List unmapped (invisible) windows."))
        .arg(Arg::with_name("wid").multiple(true))
        .get_matches();

    // Initialize xcb values
    let conn = util::init_lax("lsw");
    let screen = conn.preferred_screen();
    let root = screen.root_ref();

    // Get all passed window ids
    let wids = match args.values_of("wid") {
        Some(ids) => ids.map(util::get_window_id).collect(),
        None => vec![root.id()]
    };

    // Print requested info
    if args.is_present("r") {
        println!("0x{:08x}", root.id());
        return;
    }

    let flags = Flags {
        all   : args.is_present("a"),
        hidden: args.is_present("u"),
        ignore: args.is_present("o"),
    };

    // Print the children window IDs if applicable
    for wid in wids {
        let window = WindowRef::from(&conn, wid);
        let mut children = window.children_refs().unwrap_or_else(|_|
            panic!("cannot get children of {}", window.id())
        );

        for child in children.as_mut() {
            if should_print(child, flags) {
                println!("0x{:08x}", child.id());
            }
        }
    }
}

fn should_print(window: WindowRef, flags: Flags) -> bool {
    let attrs = window.attributes().unwrap_or_else(|_|
        panic!("could not get attributes of {}", window.id())
    );
    let mapped = attrs.map_state == MapState::Viewable;
    let ignore = attrs.override_redirect;

        flags.all
    || (!mapped && flags.hidden)
    || ( ignore && flags.ignore)
    ||      mapped
        && !ignore
        && flags.none()
}
