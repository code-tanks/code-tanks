FROM dart:stable AS build

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    git

WORKDIR /app

RUN git clone -b dart-api --single-branch https://github.com/derrick56007/codetanks.git /app

# COPY pubspec.* ./
RUN dart pub get

# Copy app source code and AOT compile it.
# COPY bin bin
# COPY lib lib
# COPY my_tank my_tank
# Ensure packages are still up-to-date if anything has changed
RUN dart pub get --offline
RUN dart compile exe bin/server.dart -o bin/server

# Build minimal serving image from AOT-compiled `/server` and required system
# libraries and configuration files stored in `/runtime/` from the build stage.
FROM scratch
COPY --from=build /runtime/ /
COPY --from=build /app/bin/server /app/bin/

# Start server.
EXPOSE 8080
CMD ["/app/bin/server"]