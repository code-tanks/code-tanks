import 'dart:io';

import 'package:codetanks/codetanks.dart';

class MyTank extends BaseTank {
  MyTank() {
    print("Created my tank!");
  }

  @override
  void run() {
    // TODO: implement run
    sleep(Duration(seconds:5));
    commands.add(Command.NONE);
  }
  
  @override
  void onEvent(Map event) {
    // TODO: implement onEvent
    print(event);
  }
}

BaseTank createTank() => MyTank();