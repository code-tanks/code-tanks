import 'package:codetanks/codetanks.dart';

class MyTank extends BaseTank {
  int i = 0;

  MyTank() {
    print("Created my spin tank!");
  }

  @override
  void run() {
    if (i % 2 == 0) {
      commands.add(Command.MOVE_FORWARD | Command.ROTATE_TANK_CLOCKWISE | Command.FIRE);
    } else {
      commands.add(Command.MOVE_BACKWARD | Command.ROTATE_TANK_COUNTER_CLOCKWISE | Command.FIRE);
    }    
    print(commands);
  }
  
  @override
  void onEvent(Map event) {
    print(event);
    i = i + 1;
    print(i);
  }
}

BaseTank createTank() => MyTank();