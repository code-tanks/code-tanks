import 'package:code_tanks/code_tanks.dart';

class MyTank extends BaseTank {
  MyTank() {
    print("Created my tank!");
  }

  @override
  void run() {}
}

BaseTank createTank() => MyTank();
