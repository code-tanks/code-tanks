import 'package:code_tanks/code_tanks.dart';

class MyTank extends BaseTank {
  MyTank() {
    print("Created my tank!");
  }

  @override
  void run() {
    // TODO: implement run
  }
  
  @override
  void onEvent(EventType e, Map info) {
    // TODO: implement onEvent
  }
}

BaseTank createTank() => MyTank();
