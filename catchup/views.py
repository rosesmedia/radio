from django.db.models import QuerySet
from django.views import generic
from django.utils import timezone

from catchup.models import Episode


class RecentEpisodesView(generic.ListView[Episode]):
    template_name = "catchup/recent_episodes.html"
    context_object_name = "episodes"

    def get_queryset(self) -> QuerySet[Episode]:
        return Episode.objects.filter(publish_at__lte=timezone.now()).order_by('publish_at')

class EpisodeView(generic.DetailView[Episode]):
    template_name = "catchup/episode.html"
    context_object_name = "episode"

    def get_queryset(self) -> QuerySet[Episode]:
        return Episode.objects.filter(publish_at__lte=timezone.now())

class UpdateEpisodeView(generic.UpdateView):
    model = Episode
    template_name = "catchup/update_episode.html"
    fields = ["name", "publish_at"]
