from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse, HTMLResponse, Response
import pandas as pd
import requests

app = FastAPI()

@app.get("/raw/{tank_id}")
def raw(tank_id: str):
    r = requests.get(f'http://server:8088/raw/{tank_id}')

    return Response(content=r.text, media_type="text/plain")

@app.get("/recent")
def recent():
    # df = pd.DataFrame([
    #     # {1: 2}, {1: 'https://google.com'}
    # ])
    # # df[1] = df[1].apply(lambda x: f'<a href="{x}">{x}</a>')
    
    # return df.to_html(escape=False)
    r = requests.get(f'http://server:8088/recent')

    return Response(content=r.json(), media_type="text/json")    

@app.get("/ping")
def root():
    return "pong"

@app.get('/{game_id}', response_class=HTMLResponse)
def index(game_id: str):
    tank_ids = game_id.split("-")
    game_id = "".join(tank_ids)

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
                    display: flex;
                    flex-direction: column-reverse;
                    align-items: center;                    
                }}

                canvas {{
                    background-color: white;
                }}

                #out {{
                    white-space: pre-wrap;
                    background: gray;
                }}

                #log {{
                    width: 1000px;
                }}
            </style>
        </head>

        <body>
            <div id="log">
                <select id="sel">
                    {
                        "".join([
                            f"<option value='{game_id}-{t}-{i}'>{t}-{i}</option>"
                            for i, t in enumerate(tank_ids)
                        ])
                    }
                </select>
                <div id="out">
                <div>
            </div>
        </body>

        <script type="module">
            import init from './{game_id}/ctviewer.js';
            init();

            var select = document.querySelector('#sel');
            var out = document.querySelector("#out");

            function display() {{
                var xmlHttp = new XMLHttpRequest();
                xmlHttp.onreadystatechange = function() {{ 
                    if (xmlHttp.readyState == 4 && xmlHttp.status == 200) {{
                        out.innerHTML = xmlHttp.responseText;
                    }}
                }};
                xmlHttp.open("GET", "http://localhost:8089/sim_log/" + select.value, true); // true for asynchronous 
                xmlHttp.send(null);
            }}


            function start(){{
                select.addEventListener('change',function(){{
                    display();
                }});
                display();
            }}

            window.addEventListener("load", start, false);

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

@app.get('/assets/sim/{game_id}')
def f4(game_id: str):
    game_id = "/".join(game_id.split(".")[0].split("-"))

    print(game_id)
    r = requests.get(f'http://server:8088/sim/{game_id}')

    return Response(content=r.text, media_type="text/plain")

app.mount("/assets", StaticFiles(directory="/ctweb/assets"), name="assets")