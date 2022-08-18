from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse, HTMLResponse

app = FastAPI()

@app.get("/ping")
def root():
    return "pong"

@app.get('/{game_id}')
def index(game_id: str):
    return f"""
        <html>

        <head>
            <meta charset="UTF-8" />
            <style>
                body {{
                    background: linear-gradient(135deg,
                            white 0%,
                            white 49%,
                            black 49%,
                            black 51%,
                            white 51%,
                            white 100%);
                    background-repeat: repeat;
                    background-size: 20px 20px;
                }}

                canvas {{
                    background-color: white;
                }}
            </style>
        </head>
        <script type="module">
            import init from './{game_id}/ctviewer.js'
            init()
        </script>

        </html>
    """

@app.get('/{game_id}/ctviewer.js')
def f1(game_id: str):
  return FileResponse('/ctweb/web/ctviewer.js')

@app.get('/{game_id}/ctviewer_bg.wasm')
def f2(game_id: str):
  return FileResponse('/ctweb/web/ctviewer_bg.wasm')

@app.get('/{game_id}/ctviewer_bg.wasm.d.ts')
def f3(game_id: str):
  return FileResponse('/ctweb/web/ctviewer_bg.wasm.d.ts')