use ct_api::*;
use serde_json::Value;

pub struct MyTank {}

impl Tank for MyTank {
    fn run(&mut self, _commands: &mut Vec<Command>) {
        todo!()
    }

    fn on_event(&mut self, _commands: &mut Vec<Command>, _event: &Value) {
        todo!()
    }
}

pub fn create_tank() -> MyTank {
    MyTank {}
}
