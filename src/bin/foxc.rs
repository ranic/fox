extern crate clap;
extern crate fox;

use clap::{App, AppSettings, Arg, SubCommand};

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::Shutdown;


fn main() {
    // TODO: Import default config (server host and port)
    let fox_addr = "127.0.0.1:1107";
    let key_arg = Arg::with_name("key").help("an ASCII key in Fox");
    let val_arg = Arg::with_name("value").help("an ASCII value in Fox");
    let set_cmd = SubCommand::with_name("set")
                    .about("Set key to a value")
                    .arg(key_arg.to_owned().required(true).index(1))
                    .arg(val_arg.to_owned().required(true).index(2));
    let get_cmd = SubCommand::with_name("get")
                    .about("Get a value for key")
                    .arg(key_arg.to_owned().required(true).index(1));
    let del_cmd = SubCommand::with_name("del") 
                    .about("Delete value for key")
                    .arg(key_arg.to_owned().required(true).index(1));
    let list_cmd = SubCommand::with_name("list").about("List all keys");

    let matches = App::new("foxc")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(set_cmd)
                    .subcommand(get_cmd)
                    .subcommand(del_cmd)
                    .subcommand(list_cmd)
                    .get_matches();

    let req = match matches.subcommand() {
        ("set", Some(r)) => fox::Message::SetReq {
            key: r.value_of("key").unwrap().to_owned(),
            value: r.value_of("value").unwrap().to_owned()
        },
        ("get", Some(r)) => fox::Message::GetReq {
            key: r.value_of("key").unwrap().to_owned(),
        },
        ("del", Some(r)) => fox::Message::DelReq {
            key: r.value_of("key").unwrap().to_owned(),
        },
        ("list", Some(_)) => fox::Message::ListReq{},
        _ => fox::Message::Undefined,
    };

    let resp = send(&fox_addr, req);
    println!("{}", resp);
}

fn send(addr: &str, req: fox::Message) -> fox::Message {
    let mut stream = TcpStream::connect(addr).unwrap();
    let mut buf = String::with_capacity(256);

    let _ = stream.write_all(req.to_string().as_ref()).unwrap();
    let _ = stream.shutdown(Shutdown::Write).unwrap();
    let _ = stream.read_to_string(&mut buf).unwrap();

    return fox::Message::new(&buf);
}
