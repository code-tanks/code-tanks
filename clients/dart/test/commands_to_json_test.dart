

import 'package:code_tanks/src/server.dart';
import 'package:code_tanks/src/base_tank.dart';
import 'package:test/test.dart';

void main() {
  test('empty', () {
    expect(
      commandsToJson([]),
      equals("[]"),
    );
  });

  test('single', () {
    expect(
      commandsToJson([Command(CommandType.None, 0)]),
      equals('[{"command_type":0,"arg":0}]'),
    );
  });

  test('double', () {
    expect(
      commandsToJson(
          [Command(CommandType.None, 0), Command(CommandType.MoveBackward, 1)]),
      equals('[{"command_type":0,"arg":0},{"command_type":2,"arg":1}]'),
    );
  });

  test('simple nested', () {
    expect(
      commandsToJson([
        Command(CommandType.None, 0),
        [
          Command(CommandType.MoveBackward, 1),
          Command(CommandType.MoveForward, 2)
        ]
      ]),
      equals(
          '[{"command_type":0,"arg":0},[{"command_type":2,"arg":1},{"command_type":1,"arg":2}]]'),
    );
  });
}
