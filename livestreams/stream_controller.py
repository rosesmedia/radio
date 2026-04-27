import requests
from typing import Any, Optional
from django.conf import settings

_STREAM_CONTROLLER_HEADERS = {
    'Authorization': 'Bearer ' + settings.STREAM_CONTROLLER_API_SECRET,
}

def _stream_controller_post(path: str, data: Optional[Any]=None) -> None:
    if data:
        resp = requests.post(settings.STREAM_CONTROLLER_API + path, json=data, headers=_STREAM_CONTROLLER_HEADERS)
    else:
        resp = requests.post(settings.STREAM_CONTROLLER_API + path, headers=_STREAM_CONTROLLER_HEADERS)
    resp.raise_for_status()

def start_stream(stream_id: str) -> None:
    _stream_controller_post('/stream/' + stream_id + '/start')

def stop_stream(stream_id: str) -> None:
    _stream_controller_post('/stream/' + stream_id + '/stop')

def set_source(stream_id: str, source: str) -> None:
    _stream_controller_post('/stream/' + stream_id + '/source', data={
        'source': source
    })

def get_source(stream_id: str) -> str:
    resp = requests.get(settings.STREAM_CONTROLLER_API + '/stream/' + stream_id + '/source', headers=_STREAM_CONTROLLER_HEADERS)
    resp.raise_for_status()
    return str(resp.json()['source'])
