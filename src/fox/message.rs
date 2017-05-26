#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag="type")]
pub enum Message {
    Undefined, // TODO: This should be removed once Message is wrapped in Result
    SetReq {key: String, value: String},
    SetResp {key: String, value: String},
    GetReq {key: String},
    GetResp {value: String},
    DelReq {key: String},
    DelResp {value: String},
    ListReq,
    ListResp {list: Vec<String>},
}


impl <> Message {
    pub fn new(data: &String) -> Message {
        // TODO: Handle errors here and return Result<Message, String>
        return serde_json::from_str(data).unwrap();
    }

    pub fn to_string(&self) -> String {
        // TODO: Handle errors here and return Result<String, String>
        return serde_json::to_string(self).unwrap();
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match self.clone() {
			Message::SetReq {key, value} => write!(f, "set {} => '{}'", key, value),
			Message::SetResp {key, value} => write!(f, "{} => '{}'", key, value),
			Message::GetReq {key} => write!(f, "get '{}'", key),
			Message::GetResp {value} => write!(f, "'{}'", value),
			Message::DelReq {key} => write!(f, "del '{}'", key),
			Message::DelResp {value} => write!(f, "'{}'", value),
			Message::ListReq => write!(f, "list"),
			Message::ListResp {list} => {
                // TODO: Ew
                let l : Vec<String> = list.iter().map(|x| format!("'{}'", x)).collect();
                return write!(f, "[{}]", l.join(", "));
            },
			_ => write!(f, "Undefined"),
		}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_json() {
        let m = Message::SetReq {key: String::from("foo"), value: String::from("value")};
        let m2 = Message::GetReq {key: String::from("foo")};

        let ser = serde_json::to_string(&m).unwrap();
        let ser2 = serde_json::to_string(&m2).unwrap();
        println!("serialized: {}", ser);
        println!("serialized: {}", ser2);

        let de : Message = serde_json::from_str(&ser).unwrap();

        println!("deserialized = {:?}", de);
    }
}
