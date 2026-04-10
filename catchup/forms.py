from django.forms.models import ModelForm

from catchup.models import Episode


class EpisodeForm(ModelForm):
    class Meta:
        model = Episode
        fields = ['name', 'slug', 'publish_at', 'original_file']
