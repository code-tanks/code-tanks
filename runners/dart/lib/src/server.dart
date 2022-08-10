import 'package:shelf_router/shelf_router.dart';
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart' as io;

import 'tank.dart';

void run(BaseTank bot) async {
  var app = Router();

  app.get('/hello', (Request request) {
    return Response.ok('hello-world');
  });

  app.get('/user/<user>', (Request request, String user) {
    return Response.ok('hello $user');
  });

  await io.serve(app, 'localhost', 8080);
}
