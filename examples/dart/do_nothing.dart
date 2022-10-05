import 'package:codetanks/codetanks.dart';

class MyTank extends BaseTank {
  MyTank() {
    print("Created my tank!");
  }

  @override
  void run() {
    // TODO: implement run
  }
  
  @override
  void onEvent(Map event) {
    // TODO: implement onEvent
    print(event);
  }
}

BaseTank createTank() => MyTank();