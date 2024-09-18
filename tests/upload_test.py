import requests
from pathlib import Path

test_dir = Path(__file__).parent

def test_status():
    url = 'http://localhost:9527/status'
    resp = requests.get(url)
    assert resp.status_code == 200


def test_echo():
    url = 'http://localhost:9527/echo'
    headers = {'Content-Type': 'text/plant; charset=utf-8'}
    resp = requests.post(url, data="abc", headers=headers)
    assert resp.status_code == 200
    assert resp.text == 'abc'


def test_upload_file():
    url = 'http://localhost:9527/upload/file-abc.txt'
    files = {'file': open(__file__, 'rb')}
    resp = requests.post(url, files=files)
    assert resp.status_code == 200, resp.text


def test_upload_html():
    url = 'http://localhost:9527/upload/index.html'
    fpath = test_dir.joinpath('index.html')
    with open(fpath, 'w') as f:
        f.write('<h1>Hello, World!</h1>')
    files = {'file': open(fpath, 'rb')}
    resp = requests.post(url, files=files)
    assert resp.status_code == 200, resp.text