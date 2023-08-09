from codetanks import BaseTank, commands
import math

class MyTank(BaseTank):
    def __init__(self):
        super().__init__()

        self.direction = 0
        self.rotation = 0
        self.x = 0
        self.y = 0
        self.gun_rotation = 0
        self.radar_rotation = 0
        self.count = 0

        print('Running my spinning tank!')

    def run(self):
        self.commands.append(commands.REQUEST_INFO | commands.MOVE_FORWARD | commands.ROTATE_GUN_CLOCKWISE | commands.ROTATE_TANK_CLOCKWISE | commands.ROTATE_RADAR_CLOCKWISE | commands.FIRE)

    def on_event(self, event):
        self.count = self.count + 1

        event_type = event["event_type"]
        if event_type == "tank_hit":
            pass
        if event_type == "radar_scan":
            print(self.count)
            if self.count < 100:
                return
            # print(event)
            # {'event_type': 'radar_scan', 'info': {'collision_type': 'Tank', 'entity': 167, 'transform': {'rotation': -3.000221014022827, 'x': 26.608129501342773, 'y': -211.02635192871094}, 'velocity': {'angvel': -0.9424778819084167, 'linvel': {'x': -99.00237274169922, 'y': -14.09011459350586}}}}    
            info = event["info"]

            collision_type = info["collision_type"]

            if collision_type == "Tank":
                # targ = math.atan2(info["transform"]["x"] - self.x, info["transform"]["y"] - self.y)
                # diff = abs(self.radar_rotation - self.gun_rotation)
                diff = self.radar_rotation - self.gun_rotation
                diff = (diff + math.pi) % (2 * math.pi) - math.pi

                cmd = commands.ROTATE_GUN_CLOCKWISE
                
                if diff > 0:
                    cmd = commands.ROTATE_GUN_COUNTER_CLOCKWISE


                diff = abs(int(diff / (math.pi * 0.3 / 60.0)))


                if diff < 0:
                    # diff = 2 * math.pi - diff
                    cmd = commands.ROTATE_GUN_CLOCKWISE

                print(diff)
                for i in range(diff):
                    self.commands.append(cmd)

                self.commands.append(commands.FIRE)
                    
                # self.direction = 1

        if event_type == "bullet_hit":
            pass
        if event_type == "request_info":
            # print(event)
            # {'event_type': 'request_info', 'info': {'gun': {'rotation': -0.015707869082689285}, 'radar': {'rotation': -0.015707869082689285}, 'tank': {'rotation': -0.015707869082689285, 'x': 161.6666717529297, 'y': -1.862645149230957e-07}}}

            info = event["info"]
            self.rotation = info["tank"]["rotation"]
            self.x = info["tank"]["x"]
            self.y = info["tank"]["y"]
            self.gun_rotation = info["gun"]["rotation"]
            self.radar_rotation = info["radar"]["rotation"]


def create_tank():
    return MyTank()