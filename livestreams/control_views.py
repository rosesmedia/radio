from django.views.generic import ListView, DetailView

from livestreams.models import Livestream


class IndexView(ListView[Livestream]):
    template_name = "livestreams/control/index.html"
    model = Livestream
    context_object_name = "livestreams"

class StreamControlView(DetailView[Livestream]):
    template_name = "livestreams/control/stream.html"
    model = Livestream
    context_object_name = "livestream"
