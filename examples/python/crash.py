from codetanks import *


class MyTank(BaseTank):
    def __init__(self):
        super().__init__()

        print('Running my tank!')
        assert True == False

    def run(self):
        # TODO
        pass

    def on_event(self, event_type, info):
        # TODO
        pass


def create_tank():
    return MyTank()