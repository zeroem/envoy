extern crate getopts;
extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use getopts::*;
use std::io;
use std::io::prelude::*;

use rustc_serialize::Encodable;
use msgpack::Encoder;

fn options() -> Options {
    let mut opts = Options::new();

    opts.optflag( "h", "help", "Print this message");
    opts.optopt("t", "type", "Message type, one of: key, keyprefix, services, nodes, service, checks, or event.", "checks");
    opts.optflag("r", "raw", "Don't wrap messages before sending");

    return opts;
}


#[derive(RustcEncodable, PartialEq, Debug)]
struct Message {
    msg_type: String,
    json: String,
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let opts = options();

    for line in stdin.lock().lines() {
        Message { msg_type: "foobar".to_string(),
                  json: line.unwrap() }.encode(&mut Encoder::new(&mut stdout));
    }
}
