from django.db.models import QuerySet
from django.views import generic
from django.utils import timezone

from catchup.models import Episode


# Create your views here.
class EpisodeView(generic.DetailView[Episode]):
    template_name = "catchup/episode.html"
    context_object_name = "episode"

    def get_queryset(self) -> QuerySet[Episode]:
        return Episode.objects.filter(publish_at__lte=timezone.now())
