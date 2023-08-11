from codetanks import BaseTank, commands
import math

ROTATE_TOGETHER_CW = commands.ROTATE_TANK_CLOCKWISE | commands.ROTATE_GUN_CLOCKWISE
ROTATE_TOGETHER_CCW = commands.ROTATE_TANK_COUNTER_CLOCKWISE | commands.ROTATE_GUN_COUNTER_CLOCKWISE

class MyTank(BaseTank):
    def __init__(self):
        super().__init__()
        print('Running my complex tank!')

    def run(self):
        # default commands when tank has no target
        self.commands = [
            commands.REQUEST_INFO | 
            commands.MOVE_FORWARD | 
            commands.ROTATE_RADAR_CLOCKWISE |
            ROTATE_TOGETHER_CW
        ]

    def on_event(self, event):
        info = event["info"]

        # save tank info for later
        if event["event_type"] == "request_info":
            self.gun_rotation = info["gun"]["rotation"]
            self.radar_rotation = info["radar"]["rotation"]
            return

        # found target
        if event["event_type"] == "radar_scan" and info["collision_type"] == "Tank":
            # find minimum angle between radar and gun
            diff = (self.radar_rotation - self.gun_rotation + math.pi) % (2 * math.pi) - math.pi

            # convert angle to number of ticks
            ticks = abs(int(diff / (math.pi * 0.3 / 60.0) / 2.0)) + 1

            # commands to rotate gun to target
            self.commands = [ROTATE_TOGETHER_CCW if diff > 0 else ROTATE_TOGETHER_CW for _ in range(ticks)]

            # unlock (prevents rotation with gun) and disable radar (prevents more radar_scan events)
            self.commands[0] |= commands.UNLOCK_RADAR | commands.DISABLE_RADAR

            # fire gun on last command, lock and enable radar
            self.commands.append(commands.FIRE | commands.LOCK_RADAR | commands.ENABLE_RADAR)


def create_tank():
    return MyTank()