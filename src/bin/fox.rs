extern crate fox;

use fox::Message;

use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;


pub struct KV<V> {
    map: HashMap<String, V>,
}

impl <V> KV<V> {
    pub fn new() -> KV<V> {
        return KV{map: HashMap::with_capacity(1024)};
    }

    pub fn get(&self, key: &String) -> Option<&V> {
        return self.map.get(key);
    }

    pub fn set(&mut self, key: String, value: V) {
        self.map.insert(key, value);
    }

    pub fn del(&mut self, key: &String) -> Option<V> {
        return self.map.remove(key);
    }
}


fn handle_message(store: &mut KV<String>, message: Message) -> Message {
    // Handle key not found same as key => ''
    match message {
        Message::SetReq {key, value} => {
            store.set(key.clone(), value.clone());
            Message::SetResp {key, value}
        },
        Message::GetReq {key} => {
            match store.get(&key) {
                // TODO: Is this clone necessary?
                Some(v) => Message::GetResp {value: v.clone()},
                None => Message::GetResp {value: String::from("")},
            }
        },
        Message::DelReq {key} => {
            match store.del(&key) {
                Some(v) => Message::DelResp {value: v},
                None => Message::DelResp {value: String::from("")},
            }
        }
        Message::ListReq => {
            // TODO: Ew
            let mut list = Vec::with_capacity(store.map.len());
            for key in store.map.keys() {
                list.push(key.clone());
            }
            Message::ListResp {list}
        }
        _ => Message::Undefined
    }
}

fn process(store: &mut KV<String>, mut stream: TcpStream) {
    let mut buf = String::with_capacity(2);
    // TODO: Catch error here
    let _ = stream.read_to_string(&mut buf);
    let m = fox::Message::new(&buf);

    let resp = handle_message(store, m);

    // TODO: Catch error here
    let _ = stream.write(resp.to_string().as_ref());
    let _ = stream.flush();
}

fn main() {
    let port = 1107;
    println!("Running server on port {}", port);

    let mut store = KV::new();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let num_reqs = listener.incoming()
                       .map(|res| {
                           res
                           .map(|stream| process(&mut store, stream))
                           .map_err(|e| println!("Listen failed: {}", e))
                       })
                       .count();

    println!("Done, served {} requests!", num_reqs);
	/* Above is equivalent to this, but with Monads (for fun)
    for stream in listener.incoming() {
		match stream {
			Ok(s) => {/* handle ok */},
			Err(e) => {/* handle err*/}
		}
	}*/
}
