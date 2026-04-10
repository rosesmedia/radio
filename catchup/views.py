from django.contrib import messages
from django.contrib.auth.decorators import permission_required
from django.contrib.auth.mixins import PermissionRequiredMixin
from django.db.models import QuerySet
from django.forms import inlineformset_factory
from django.forms.models import ModelForm
from django.shortcuts import get_object_or_404, redirect, render
from django.utils import timezone
from django.utils.translation import gettext_lazy as _
from django.views import generic

from catchup import tasks
from catchup.forms import EpisodeForm
from catchup.models import Episode, EpisodeTag


class RecentEpisodesView(generic.ListView[Episode]):
    template_name = "catchup/recent_episodes.html"
    context_object_name = "episodes"

    def get_queryset(self) -> QuerySet[Episode]:
        return Episode.objects.filter(publish_at__lte=timezone.now()).order_by('-publish_at')

class EpisodeView(generic.DetailView[Episode]):
    template_name = "catchup/episode.html"
    context_object_name = "episode"

    def get_queryset(self) -> QuerySet[Episode]:
        return Episode.objects.filter(publish_at__lte=timezone.now())

def create_episode(request):
    InlineFormSet = inlineformset_factory(Episode, EpisodeTag, fields=('tag',), extra=3)
    form = EpisodeForm(request.POST or None, request.FILES or None)
    formset = InlineFormSet(request.POST or None, instance=Episode())
    if form.is_valid() and formset.is_valid():
        episode = form.save()
        formset.instance = episode
        formset.save()
        tasks.process_upload.delay(episode.pk, True)
        return redirect('catchup:detail', episode.slug)
    ctx = {
        'form': form,
        'formset': formset,
    }
    return render(request, 'catchup/create_episode.html', ctx)

class UpdateEpisodeView(PermissionRequiredMixin, generic.UpdateView[Episode, ModelForm[Episode]]):
    model = Episode
    template_name = "catchup/update_episode.html"
    fields = ["name", "publish_at"]
    permission_required = "catchup.change_episode"
    raise_exception = True

@permission_required("catchup.change_episode", raise_exception=True)
def update_episode_tags(request, slug):
    InlineFormSet = inlineformset_factory(Episode, EpisodeTag, fields=('tag',), extra=1)
    episode = get_object_or_404(Episode, slug=slug)
    formset = InlineFormSet(request.POST or None, instance=episode, )
    if formset.is_valid():
        formset.instance = episode
        formset.save()
        messages.success(request, _("Tags updated!"))
        return redirect('catchup:detail', slug)
    ctx = {
        'episode': episode,
        'formset': formset,
    }
    return render(request, 'catchup/update_episode_tasks.html', ctx)
