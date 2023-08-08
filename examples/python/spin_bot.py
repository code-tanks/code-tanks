from codetanks import BaseTank, commands

class MyTank(BaseTank):
    def __init__(self):
        super().__init__()

        self.direction = 0

        print('Running my spinning tank!')

    def run(self):
        if self.direction % 2 == 0:
            self.commands.append(commands.MOVE_BACKWARD | commands.ROTATE_TANK_COUNTER_CLOCKWISE | commands.FIRE)
        else:
            self.commands.append(commands.MOVE_FORWARD | commands.ROTATE_TANK_CLOCKWISE | commands.FIRE)

    def on_event(self, event):
        self.direction = self.direction + 1

def create_tank():
    return MyTank()