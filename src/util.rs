extern crate xcb;
extern crate lax;

use std::process;
use self::lax::prelude::*;

pub fn init_xcb(programname: &str) -> xcb::Connection {
    init_lax(programname).into_xcb()
}

pub fn init_lax(programname: &str) -> Connection {
    match Connection::new() {
        Ok(conn) => conn,
        Err(_) => {
            println!("{}: Unable to connect to the X server", programname);
            process::exit(1);
        }
    }
}

pub fn get_screen<'a>(setup: &'a xcb::Setup) -> xcb::Screen<'a> {
    setup.roots().next().expect("Lost connection to X server")
}

pub fn exists(conn: &xcb::Connection, window: xcb::Window) -> bool {
    let win_attrib_cookie = xcb::get_window_attributes(&conn, window);
    let win_attrib_cookie_reply_result = win_attrib_cookie.get_reply();

    match win_attrib_cookie_reply_result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn mapped(conn: &xcb::Connection, window: xcb::Window) -> bool {
    let attrs = xcb::get_window_attributes(&conn, window).get_reply();
    match attrs {
        Ok(attrs) => attrs.map_state() as u32 == xcb::MAP_STATE_VIEWABLE,
        _ => false
    }
}

pub fn ignore(conn: &xcb::Connection, window: xcb::Window) -> bool {
    let attrs = xcb::get_window_attributes(&conn, window).get_reply();
    match attrs {
        Ok(attrs) => attrs.override_redirect(),
        _ => false
    }
}

pub fn get_window_id(input: &str) -> xcb::Window {
    let window = if input.starts_with("0x") {
        &input[2..]
    } else {
        input
    };

    match u32::from_str_radix(window, 16) {
        Ok(val) => val,
        Err(_) => 0,
    }
}

pub fn get_query_tree(conn: &xcb::Connection, window: xcb::Window) -> xcb::QueryTreeReply {
    xcb::query_tree(conn, window).get_reply().expect("no such window")
}
