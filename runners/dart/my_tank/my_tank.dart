import 'package:code_tanks/code_tanks.dart';

class MyTank extends BaseTank {
  @override
  void run() {}
}

BaseTank createTank() => MyTank();
