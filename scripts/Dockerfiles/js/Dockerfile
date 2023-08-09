FROM node:20

WORKDIR /app

# RUN git clone -b 'v0.1.1' --single-branch --depth 1 https://github.com/code-tanks/code-tanks.git /app
RUN git clone -b 'v0.0.2' --single-branch --depth 1 https://github.com/code-tanks/javascript-api.git

WORKDIR /app/javascript-api
ARG url

COPY $url tanks/my_tank.js
RUN sed -i 's#javascript-api#../lib/api.js#' tanks/my_tank.js

# Start server.
EXPOSE 8080
CMD ["node", "server.js"]
