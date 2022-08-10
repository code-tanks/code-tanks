import 'package:code_tanks/code_tanks.dart';

class DoNothingTank extends BaseTank {
  @override
  void run() {
    commands.add(Command(CommandType.None, 0));
  }

  @override
  void onEvent(EventType e, Map info) {
    commands.addAll([
      Command(CommandType.None, 0),
      [
        Command(CommandType.None, 0),
        Command(CommandType.None, 0),
      ]
    ]);
  }
}
