from django.http import HttpRequest, HttpResponse, JsonResponse
from django.shortcuts import get_object_or_404

from livestreams.models import Livestream


def stream_config_view(_request: HttpRequest, slug: str) -> HttpResponse:
    stream = get_object_or_404(Livestream, slug=slug)

    return JsonResponse({
        'ingest_url': stream.ingest_point.icecast_url if stream.ingest_point else None,
    })
