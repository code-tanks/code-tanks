import 'package:codetanks/codetanks.dart';

class MyTank extends BaseTank {
  int i = 0;

  MyTank() {
    print("Created my tank!");
  }

  @override
  void run() {
    if (i == 0) {
      commands.add(Command.MOVE_FORWARD);
    }
    commands.add(Command.NONE);
    i = i + 1;
  }
  
  @override
  void onEvent(Map event) {
  }
}

BaseTank createTank() => MyTank();