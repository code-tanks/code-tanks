FROM python:3.10-slim

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    git curl

WORKDIR /app

RUN git clone -b 'v0.1.1' --single-branch --depth 1 https://github.com/code-tanks/python-api.git /app

RUN pip install -r requirements.txt

ARG url

RUN curl http://localhost:8089/raw/$url > tanks/my_tank.py

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8080"]