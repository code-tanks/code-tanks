import 'package:codetanks/codetanks.dart';

class MyTank extends BaseTank {
  MyTank() {
    print("Created my tank!");
  }

  @override
  void run() {
    commands.add(Command.MOVE_FORWARD | Command.ROTATE_GUN_CLOCKWISE);
  }
  
  @override
  void onEvent(Map event) {
  }
}

BaseTank createTank() => MyTank();