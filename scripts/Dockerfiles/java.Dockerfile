FROM eclipse-temurin:17-jdk-jammy AS builder

RUN apt update && apt install -y git maven

WORKDIR /app

RUN git clone -b 'v0.0.2' --single-branch --depth 1 https://github.com/code-tanks/java-api.git
# RUN git clone https://github.com/code-tanks/java-api.git

WORKDIR /app/java-api

RUN mvn compile assembly:single

ARG url

COPY $url src/main/java/tanks/MyTank.java

RUN mvn compile assembly:single

FROM eclipse-temurin:17-jdk-jammy

WORKDIR /app

COPY --from=builder /app/java-api/target/my-app-1.0-SNAPSHOT-jar-with-dependencies.jar /app/app.jar

EXPOSE 8080

CMD [ "java", "-jar", "app.jar" ]

