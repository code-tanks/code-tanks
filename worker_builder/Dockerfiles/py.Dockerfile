FROM python:3.10

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    git curl

WORKDIR /app

RUN git clone --depth 1 https://github.com/code-tanks/python-api.git /app

RUN pip install -r requirements.txt

RUN curl http://localhost:8089/raw/$url > tanks/my_tank.py


CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8080"]