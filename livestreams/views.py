# Create your views here.
from django.db.models import QuerySet
from django.views import generic

from livestreams.models import Livestream

class LiveStreamsView(generic.ListView[Livestream]):
    template_name = "livestreams/list.html"
    model = Livestream
    context_object_name = "streams"

    def get_queryset(self) -> QuerySet[Livestream]:
        return Livestream.objects.filter(ended_at__isnull=True, started_at__isnull=False)

class LivestreamView(generic.DetailView[Livestream]):
    template_name = "livestreams/stream.html"
    model = Livestream
    context_object_name = "stream"
