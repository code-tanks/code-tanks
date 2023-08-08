# Build stage
FROM golang:1.16 AS build

WORKDIR /app

RUN git clone -b 'v0.0.1' --single-branch --depth 1 https://github.com/code-tanks/golang-api.git /app

COPY go.mod go.sum ./
RUN go mod download

ARG url

COPY $url pkg/tanks/my_tank.go

RUN CGO_ENABLED=0 GOOS=linux go build -o main

# Final stage
FROM scratch

COPY --from=build /app/main /

ENTRYPOINT ["/main"]
