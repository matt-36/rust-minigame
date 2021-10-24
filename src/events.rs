use serde_json::{self, Value, json, Number};
use serde::{Deserialize, Serialize};



#[derive(Clone)]
pub struct EventHandler {
    incoming: Vec<Value>,
    outgoing: Vec<Value>
}

impl<'a> EventHandler {
    pub fn Init() -> EventHandler {
        EventHandler {
            incoming: Vec::new(),
            outgoing: Vec::new()
        }
    }

    pub fn handle(mut self, event: Value) -> Result<Value, ()>{
        match event["type"].as_u64().expect("failed to parse data event") {
            0 => {
                let x = json!({
                    "type": 0,
                    "message": "im here",
                });
                self.send(x.clone());
                Ok(x)
            },
            1 | 2 => {
                Ok(event)
            }

            _ => {
                panic!("Bad event: {}", event["data"])
            }
        }
    }

    pub fn send(mut self, event: Value) {
        self.outgoing.push(event);
    }
    pub fn recv(mut self, event: Value) {
        self.incoming.push(event)
    }

    pub fn IterIncoming(mut self) -> Vec<Value> {
        let c = self.incoming.clone();
        self.incoming.clear();
        c
    }
}