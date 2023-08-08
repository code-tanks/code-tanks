from codetanks import BaseTank, commands

class MyTank(BaseTank):
    def __init__(self):
        super().__init__()

        print('Running my tank!')
        assert True == False

    def run(self):
        # TODO
        pass

    def on_event(self, event):
        # TODO
        pass


def create_tank():
    return MyTank()