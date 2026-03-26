from django.views import generic

from catchup.models import Episode


# Create your views here.
class EpisodeView(generic.DetailView):
    template_name = "catchup/episode.html"
    model = Episode
