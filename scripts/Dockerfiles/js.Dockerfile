FROM node:20

WORKDIR /app

# RUN git clone -b 'v0.1.1' --single-branch --depth 1 https://github.com/code-tanks/code-tanks.git /app
RUN git clone -b 'v0.0.1' --single-branch --depth 1 https://github.com/code-tanks/javascript-api.git
RUN npm install javascript-api

ARG url

COPY $url tanks/my_tank.js

# Start server.
EXPOSE 8080
CMD ["node", "/app/javascript-api/server.js"]
