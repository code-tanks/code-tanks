FROM python:3.10-slim

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    git curl

WORKDIR /app

RUN git clone -b 'v0.1.3' --single-branch --depth 1 https://github.com/code-tanks/python-api.git /app

RUN pip install -r requirements.txt

ARG url

COPY url tanks/my_tank.py

# RUN curl server:8088/raw/$url > tanks/my_tank.py

EXPOSE 8080
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8080"]