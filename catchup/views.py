from django.views import generic
from django.utils import timezone

from catchup.models import Episode


# Create your views here.
class EpisodeView(generic.DetailView):
    template_name = "catchup/episode.html"
    context_object_name = "episode"

    def get_queryset(self):
        return Episode.objects.filter(publish_at__lte=timezone.now())
