from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse

app = FastAPI()

@app.get("/ping")
def root():
    return "pong"

@app.get('/')
def index():
  return FileResponse('/ctweb/web/index.html')

@app.get('/ctviewer.js')
def f1():
  return FileResponse('/ctweb/web/ctviewer.js')

@app.get('/ctviewer_bg.wasm')
def f2():
  return FileResponse('/ctweb/web/ctviewer_bg.wasm')

@app.get('/ctviewer_bg.wasm.d.ts')
def f3():
  return FileResponse('/ctweb/web/ctviewer_bg.wasm.d.ts')