extern crate getopts;
extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use std::env;
use std::io;
use std::io::prelude::*;

use getopts::*;
use msgpack::Encoder;
use rustc_serialize::Encodable;

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

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("{} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts = options();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    if matches.opt_present("raw") {
       // Pipe stdin directly to the writer
    } else {
        for line in stdin.lock().lines() {
            Message { msg_type: "foobar".to_string(),
                      json: line.unwrap() }.encode(&mut Encoder::new(&mut stdout));
        }
    }
}
