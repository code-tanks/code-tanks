import 'package:codetanks/codetanks.dart';

class MyTank extends BaseTank {
  int i = 0;

  MyTank() {
    print("Created my tank!");
  }

  @override
  void run() {
    if (i == 0) {
      for (int j = 0; j < 200; j++) {
        commands.add(Command.ROTATE_TANK_CLOCKWISE);
      }
    } else {
      commands.add(Command.MOVE_BACKWARD | Command.ROTATE_TANK_COUNTER_CLOCKWISE | Command.FIRE);
    }    
    print(commands);
    i = i + 1;
  }
  
  @override
  void onEvent(Map event) {
    print(event);
  }
}

BaseTank createTank() => MyTank();