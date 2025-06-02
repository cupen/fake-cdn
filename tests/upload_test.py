import requests
from pathlib import Path
import uuid
import os

ENV = lambda x, _default: os.environ.get(x, _default)

base_url = ENV('BASE_URL', 'http://localhost:9527')

TOKEN = "123456"
test_dir = Path(__file__).parent

class File:
    @staticmethod
    def upload(fpath, url_path, token=TOKEN):
        files = {'file': open(fpath, 'rb')}
        url = f'{base_url}/{url_path}'
        if not token:
            return requests.post(url, files=files)
        headers = {'Authorization': token}
        return requests.post(url, files=files, headers=headers)

    @staticmethod
    def download(url_path):
        url = f'{base_url}/{url_path}'
        return requests.get(url)

    @staticmethod
    def create(name, content):
        fpath = test_dir / ".data" / name
        os.makedirs(fpath.parent, exist_ok=True)
        with open(fpath, 'w') as f:
            f.write(content)
        return fpath


def test_upload_token():
    resp = File.upload(__file__, 'file-abc.txt', token='invalid token')
    assert resp.status_code == 401, resp.text

    resp = File.upload(__file__, 'file-abc.txt', token='')
    assert resp.status_code == 401, resp.text


def test_status():
    url = f'{base_url}/status'
    resp = requests.get(url)
    assert resp.status_code == 200
    assert resp.json() == {'status': 'ok', 'version': '0.1.0'}


def test_upload_file():
    id = str(uuid.uuid4())
    resp = File.upload(__file__, f"{id}/file-abc.txt")
    assert resp.status_code == 200, resp.text

    resp = File.download(f"{id}/file-abc.txt")
    assert resp.status_code == 200 
    assert resp.headers['Content-Type'] == 'text/plain; charset=utf-8'


def test_upload_html():
    id = str(uuid.uuid4())
    fpath = test_dir.joinpath('index.html')
    with open(fpath, 'w') as f:
        f.write('<h1>Hello, World!</h1>')
    resp = File.upload(fpath, f"{id}/index.html")
    assert resp.status_code == 200, resp.text

    resp = File.download(f"{id}/index.html")
    assert resp.status_code == 200 
    assert resp.text == '<h1>Hello, World!</h1>'
    assert resp.headers['Content-Type'] == 'text/html; charset=utf-8'


def test_upload_html_override():
    id = str(uuid.uuid4())
    def do(content):
        fpath = File.create('override/index.html', content)
        resp = File.upload(fpath, f"{id}/index.html")
        assert resp.status_code == 200, resp.text

        resp = File.download(f"{id}/index.html")
        assert resp.status_code == 200 
        assert resp.text == content
        assert resp.headers['Content-Type'] == 'text/html; charset=utf-8'
        pass

    do(f'<h1>Hello, World!</h1>')
    do(f'<h1>Hello, World!</h1> {id}')
    do(f'<h1>Hello, World!</h1> {id} again')
    pass


def test_upload_tgz():
    F = File.create
    id = str(uuid.uuid4())
    url_paths = [
        'tar/index.html',
        'tar/css/abc.css',
        'tar/js/abc.js',
    ]
    # create a tar file
    fpath = F('index.html', '<h1>Hello, World!</h1>')
    import tarfile
    with tarfile.open(fpath.with_suffix('.tar.gz'), 'w:gz') as tar:
        tar.add(F("index.html", '<h1>Hello, World!</h1>'), arcname='index.html')
        tar.add(F("abc.css", 'abc'), arcname='css/abc.css')
        tar.add(F("abc.js", '{}'), arcname='js/abc.js')
        pass

    resp = File.upload(fpath.with_suffix('.tar.gz'), f'tar/index.tar.gz')
    assert resp.status_code == 200 

    for path in url_paths:
        resp = File.download(path)
        assert resp.status_code == 200 
    pass
