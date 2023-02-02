use ct_api::*;
use serde_json::Value;

pub struct MyTank {
    i: u32
}

impl Tank for MyTank {
    fn run(&mut self, commands: &mut Vec<Command>) {
        if self.i % 2 == 0 {
            commands.push(Commands::MOVE_FORWARD | Commands::ROTATE_TANK_CLOCKWISE | Commands::FIRE);
        } else {
            commands.push(Commands::MOVE_BACKWARD | Commands::ROTATE_TANK_COUNTER_CLOCKWISE | Commands::FIRE);
        }
    }

    fn on_event(&mut self, commands: &mut Vec<Command>, event: &Value) {
        self.i += 1;
    }
}

pub fn create_tank() -> MyTank {
    MyTank { i: 0 }
}
