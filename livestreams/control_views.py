from typing import Any

from django.contrib.auth.decorators import permission_required
from django.contrib.auth.mixins import PermissionRequiredMixin
from django.http import HttpRequest, HttpResponse, HttpResponseRedirect, HttpResponseNotFound
from django.shortcuts import get_object_or_404, render
from django.urls import reverse
from django.utils import timezone
from django.views.generic import ListView, DetailView

from livestreams import forms, stream_controller
from livestreams.models import Livestream


class IndexView(PermissionRequiredMixin, ListView[Livestream]):
    template_name = "livestreams/control/index.html"
    model = Livestream
    context_object_name = "livestreams"
    permission_required = "livestreams.control_livestream"

class StreamControlView(PermissionRequiredMixin, DetailView[Livestream]):
    template_name = "livestreams/control/stream.html"
    model = Livestream
    context_object_name = "livestream"
    permission_required = "livestreams.control_livestream"

    def get_context_data(self, **kwargs: Any) -> dict[str, Any]:
        context = super(StreamControlView, self).get_context_data(**kwargs)
        context['selected'] = self.request.GET.get('selected', None)
        return context

@permission_required("livestreams.control_livestream", raise_exception=True)
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

@permission_required("livestreams.control_livestream", raise_exception=True)
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

@permission_required("livestreams.control_livestream", raise_exception=True)
def stream_source(request: HttpRequest, slug: str) -> HttpResponse:
    stream = get_object_or_404(Livestream, slug=slug)
    if stream.started_at is None or stream.ended_at is not None:
        return HttpResponseNotFound()
    if request.method == 'GET':
        selected = request.GET.get('selected', None)
        current = stream_controller.get_source(stream.slug)
        sources = []
        for name, source_id in stream_controller.SOURCES.items():
            sources.append({
                'is_live': source_id == current,
                'is_selected': source_id == selected,
                'name': name,
                'id': source_id,
            })
        return render(request, 'livestreams/control/source_select.html', {
            'sources': sources,
            'livestream': stream,
        })
    elif request.method == 'POST':
        form = forms.SourceForm(request.POST)
        if form.is_valid():
            stream_controller.set_source(stream.slug, form['source'].value())
            return HttpResponseRedirect(reverse('livestreams:control_stream', args=(slug,)))
    return HttpResponseNotFound()
