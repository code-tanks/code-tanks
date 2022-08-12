FROM dart:stable AS build

ARG url

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    git curl

WORKDIR /app

RUN git clone -b dart-api --single-branch https://github.com/derrick56007/codetanks.git /app

RUN dart pub get

RUN curl http://localhost:8089/url > tanks/my_tank.dart

RUN dart compile exe bin/server.dart -o bin/server

# Build minimal serving image from AOT-compiled `/server` and required system
# libraries and configuration files stored in `/runtime/` from the build stage.
FROM scratch
COPY --from=build /runtime/ /
COPY --from=build /app/bin/server /app/bin/

# Start server.
EXPOSE 8080
CMD ["/app/bin/server"]
