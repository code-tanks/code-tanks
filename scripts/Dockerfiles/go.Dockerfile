# Build stage
FROM golang:latest AS build

WORKDIR /app

RUN git clone -b 'v0.0.1' --single-branch --depth 1 https://github.com/code-tanks/golang-api.git

WORKDIR /app/golang-api

# COPY go.mod go.sum .
RUN go mod download

RUN CGO_ENABLED=0 GOOS=linux go build -o main web/main.go

ARG url

COPY $url pkg/tanks/my_tank.go

RUN CGO_ENABLED=0 GOOS=linux go build -o main web/main.go

# Final stage
FROM scratch

COPY --from=build /app/golang-api/main /

ENTRYPOINT ["/main"]
