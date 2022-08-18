from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

app = FastAPI()

@app.get("/ping")
async def root():
    return "pong"

app.mount("/", StaticFiles(directory="/ctweb/web"), name="static")