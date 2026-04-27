use serde::{Deserialize, Serialize};
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct ChatMsg {
    pub sender: String,
    pub msg: String,
}

impl ChatMsg {
    pub fn new (msg:String,sender:String) -> Self {
        ChatMsg { sender, msg }
    }
    
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct ChatLog {
    pub log: Vec<ChatMsg>, 
}

impl ChatLog {
    pub fn new () -> Self{
        ChatLog { log: Vec::new() }
    }
    pub fn add_msg(&mut self,msg:ChatMsg) {
        self.log.push(msg);
    }
}
