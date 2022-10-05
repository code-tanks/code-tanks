import 'package:codetanks/codetanks.dart';

class MyTank extends BaseTank {
  int i = 0;

  MyTank() {
    print("Created my spin tank!");
  }

  @override
  void run() {
    if (i % 2 == 0) {
        for (var j = 0; j < 1000; j++) {
          commands.add(Command.MOVE_FORWARD | Command.FIRE);
        }
    } else {
        for (var j = 0; j < 1000; j++) {
          commands.add(Command.MOVE_BACKWARD| Command.FIRE);
        }
    }
    i = i + 1;
  }
  
  @override
  void onEvent(Map event) {
    print(event);

  }
}

BaseTank createTank() => MyTank();