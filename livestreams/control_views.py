from django.http import HttpRequest, HttpResponse, HttpResponseRedirect
from django.shortcuts import get_object_or_404
from django.urls import reverse
from django.utils import timezone
from django.views.generic import ListView, DetailView

from livestreams import forms, stream_controller
from livestreams.models import Livestream


class IndexView(ListView[Livestream]):
    template_name = "livestreams/control/index.html"
    model = Livestream
    context_object_name = "livestreams"

class StreamControlView(DetailView[Livestream]):
    template_name = "livestreams/control/stream.html"
    model = Livestream
    context_object_name = "livestream"

def start_stream(request: HttpRequest, slug: str) -> HttpResponse:
    stream = get_object_or_404(Livestream, slug=slug)
    if stream.started_at is None and stream.ingest_point is not None:
        if request.method == 'POST':
            form = forms.EmptyForm(request.POST)
            if form.is_valid():
                stream_controller.start_stream(stream.slug)
                stream.started_at = timezone.now()
                stream.save()

    return HttpResponseRedirect(reverse('livestreams:control_stream', args=(slug,)))

def stop_stream(request: HttpRequest, slug: str) -> HttpResponse:
    stream = get_object_or_404(Livestream, slug=slug)
    if stream.started_at is not None and stream.ended_at is None:
        if request.method == 'POST':
            form = forms.EmptyForm(request.POST)
            if form.is_valid():
                stream_controller.stop_stream(stream.slug)
                stream.ended_at = timezone.now()
                stream.save()

    return HttpResponseRedirect(reverse('livestreams:control_stream', args=(slug,)))
