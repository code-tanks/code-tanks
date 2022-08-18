from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse

app = FastAPI()

@app.get("/ping")
async def root():
    return "pong"

@app.get('/')
def index():
  return FileResponse('/ctweb/web/index.html')