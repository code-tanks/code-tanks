use ct_api::*;
use serde_json::Value;

pub struct MyTank {
    i: u32
}

impl Tank for MyTank {
    fn run(&mut self, commands: &mut Vec<CCommand>) {
        if self.i % 2 == 0 {
            commands.push(CCommands::MOVE_FORWARD | CCommands::ROTATE_TANK_CLOCKWISE | CCommands::FIRE);
        } else {
            commands.push(CCommands::MOVE_BACKWARD | CCommands::ROTATE_TANK_COUNTER_CLOCKWISE | CCommands::FIRE);
        }
    }

    fn on_event(&mut self, commands: &mut Vec<CCommand>, event: &Value) {
        self.i += 1;
    }
}

pub fn create_tank() -> MyTank {
    MyTank { i: 0 }
}
