import 'package:codetanks/codetanks.dart';

class MyTank extends BaseTank {
  int i = 0;

  MyTank() {
    print("Created my spin tank!");
  }

  @override
  void run() {
    commands.add(Commands.MOVE_FORWARD | Commands.ROTATE_TANK_CLOCKWISE | Commands.FIRE);
  }
  
  @override
  void onEvent(EventType e, Map info) {
    if (i % 2 == 0) {
        commands.add(Commands.MOVE_BACKWARD | Commands.ROTATE_TANK_COUNTER_CLOCKWISE | Commands.FIRE);
    } else {
        commands.add(Commands.MOVE_FORWARD | Commands.ROTATE_TANK_CLOCKWISE | Commands.FIRE);
    }
    i = i + 1;
  }
}

BaseTank createTank() => MyTank();