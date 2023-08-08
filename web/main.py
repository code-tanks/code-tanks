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

@app.get("/log/{tank_id}")
def log(tank_id: str):
    r = requests.get(f'http://server:8088/log/{tank_id}')

    return Response(content=r.text, media_type="text/plain")

@app.get("/sim_log/{tank_id}")
def log(tank_id: str):
    r = requests.get(f'http://server:8088/sim_log/{tank_id}')

    return Response(content=r.text, media_type="text/plain")    

@app.get("/recent")
def recent():
    r = requests.get(f'http://server:8088/recent')
    j = r.json()
    print(j)
    df = pd.DataFrame(j)
    df['id'] = df['id'].apply(lambda x: f'<a href="/{x}">{x}</a>')
    df['tanks'] = df['tanks'].apply(lambda x: [f'<a href="/raw/{i}">{i}</a>' for i in x])
    
    return Response(content=df.to_html(escape=False), media_type="text/html")    

@app.get("/ping")
def root():
    return "pong"

@app.get('/view/{game_url}', response_class=HTMLResponse)
def index(game_url: str):
    tank_ids = game_url.split("-")
    # game_url = "".join(tank_ids)

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
                            f"<option value='{game_url}-{t}-{i}'>{t}-{i}</option>"
                            for i, t in enumerate(tank_ids)
                        ])
                    }
                </select>
                <div id="out">
                <div>
            </div>
        </body>

        <script type="module">
            import init from '/ctviewer.js';
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
                xmlHttp.open("GET", "/sim_log/" + select.value, true); // true for asynchronous 
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

@app.get('/ctviewer.js')
def f1():
    res = FileResponse('/ctweb/web/ctviewer.js')
    res.headers["Cache-Control"]  = "public, max-age=604800"
    return res

@app.get('/ctviewer_bg.wasm')
def f2():
    res = FileResponse('/ctweb/web/ctviewer_bg.wasm')
    res.headers["Cache-Control"]  = "public, max-age=604800"
    return res
@app.get('/assets/ctviewer_bg.wasm.d.ts')
def f3():
    res = FileResponse('/ctweb/web/ctviewer_bg.wasm.d.ts')
    res.headers["Cache-Control"]  = "public, max-age=604800"
    return res

@app.get('/sim/{game_url}')
def f4(game_url: str):
    game_url = game_url[5:-4]
    print(1, game_url)
    r = requests.get(f'http://server:8088/sim/{game_url}')
    print(2, r.text)

    res = Response(content=r.text, media_type="text/plain")
    res.headers["Cache-Control"]  = "public, max-age=604800"
    return res

app.mount("/view/assets", StaticFiles(directory="/ctweb/assets"), name="assets")