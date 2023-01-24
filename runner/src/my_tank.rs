use ct_api::*;
use serde_json::Value;

pub struct MyTank {}

impl Tank for MyTank {
    fn run(&mut self, commands: &mut Vec<Command>) {
        todo!()
    }

    fn on_event(&mut self, commands: &mut Vec<Command>, event: &Value) {
        todo!()
    }
}

pub fn create_tank() -> MyTank {
    MyTank {}
}
