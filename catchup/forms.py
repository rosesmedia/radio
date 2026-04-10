from django.forms.models import ModelForm, inlineformset_factory, BaseInlineFormSet

from catchup.models import Episode, EpisodeTag


class EpisodeForm(ModelForm[Episode]):
    class Meta:
        model = Episode
        fields = ['name', 'slug', 'publish_at', 'original_file']

CreateEpisodeTagsFormset: type[BaseInlineFormSet[EpisodeTag, Episode, ModelForm[EpisodeTag]]] = inlineformset_factory(Episode, EpisodeTag, fields=('tag',), extra=3)
UpdateEpisodeTagsFormset: type[BaseInlineFormSet[EpisodeTag, Episode, ModelForm[EpisodeTag]]] = inlineformset_factory(Episode, EpisodeTag, fields=('tag',), extra=1)
