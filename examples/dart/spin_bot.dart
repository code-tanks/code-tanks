import 'package:codetanks/codetanks.dart';

class MyTank extends BaseTank {
  int i = 0;

  MyTank() {
    print("Created my spin tank!");
  }

  @override
  void run() {
    commands.add(Command.MOVE_FORWARD | Command.ROTATE_TANK_CLOCKWISE | Command.FIRE);
  }
  
  @override
  void onEvent(EventType e, Map info) {
    if (i % 2 == 0) {
        commands.add(Command.MOVE_BACKWARD | Command.ROTATE_TANK_COUNTER_CLOCKWISE | Command.FIRE);
    } else {
        commands.add(Command.MOVE_FORWARD | Command.ROTATE_TANK_CLOCKWISE | Command.FIRE);
    }
    i = i + 1;
  }
}

BaseTank createTank() => MyTank();