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
    print("$e $info");
    if (i % 2 == 0) {
        for (var j = 0; j < 1000; j++) {
            commands.add(Command.MOVE_BACKWARD | Command.ROTATE_TANK_COUNTER_CLOCKWISE | Command.FIRE);
        }
    }
    i = i + 1;
  }
}

BaseTank createTank() => MyTank();