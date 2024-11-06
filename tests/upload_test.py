import requests
from pathlib import Path
import uuid

test_dir = Path(__file__).parent

def test_status():
    url = 'http://localhost:9527/status'
    resp = requests.get(url)
    assert resp.status_code == 200


def test_upload_file():
    id = str(uuid.uuid4())
    url = f'http://localhost:9527/{id}/file-abc.txt'
    files = {'file': open(__file__, 'rb')}
    resp = requests.post(url, files=files)
    assert resp.status_code == 200, resp.text

    resp = requests.get(url)
    assert resp.status_code == 200 
    assert resp.headers['Content-Type'] == 'text/plain; charset=utf-8'


def test_upload_html():
    id = str(uuid.uuid4())
    url = f'http://localhost:9527/{id}/index.html'
    fpath = test_dir.joinpath('index.html')
    with open(fpath, 'w') as f:
        f.write('<h1>Hello, World!</h1>')
    files = {'file': open(fpath, 'rb')}
    resp = requests.post(url, files=files)
    assert resp.status_code == 200, resp.text

    resp = requests.get(url)
    assert resp.status_code == 200 
    assert resp.text == '<h1>Hello, World!</h1>'
    assert resp.headers['Content-Type'] == 'text/html; charset=utf-8'


def test_upload_html_override():
    id = str(uuid.uuid4())
    url = f'http://localhost:9527/override/index.html'
    def do(content):
        fpath = test_dir.joinpath('index.html')
        with open(fpath, 'w') as f:
            f.write(content)
        files = {'file': open(fpath, 'rb')}
        resp = requests.post(url, files=files)
        assert resp.status_code == 200, resp.text

        resp = requests.get(url)
        assert resp.status_code == 200 
        assert resp.text == content
        assert resp.headers['Content-Type'] == 'text/html; charset=utf-8'
        pass

    do(f'<h1>Hello, World!</h1>')
    do(f'<h1>Hello, World!</h1> {id}')
    do(f'<h1>Hello, World!</h1> {id} again')
    pass

def create_file(name, content):
    fpath = test_dir.joinpath(name)
    with open(fpath, 'w') as f:
        f.write(content)
    return fpath

def test_upload_tar():
    F = create_file
    id = str(uuid.uuid4())
    url = f'http://localhost:9527/tar/index.tar.gz'
    url_files = [
        'http://localhost:9527/tar/index.html',
        'http://localhost:9527/tar/css/abc.css',
        'http://localhost:9527/tar/js/abc.js',
    ]
    # create a tar file
    fpath = F('index.html', '<h1>Hello, World!</h1>')
    import tarfile
    with tarfile.open(fpath.with_suffix('.tar.gz'), 'w:gz') as tar:
        tar.add(F("index.html", '<h1>Hello, World!</h1>'), arcname='index.html')
        tar.add(F("abc.css", 'abc'), arcname='css/abc.css')
        tar.add(F("abc.js", '{}'), arcname='js/abc.js')

    files = {'file': open(fpath.with_suffix('.tar.gz'), 'rb')}
    resp = requests.post(url, files=files)
    assert resp.status_code == 200 

    for url in url_files:
        resp = requests.get(url)
        assert resp.status_code == 200 
        pass
    pass