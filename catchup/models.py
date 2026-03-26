from django.db import models
from django.utils import timezone
from django.utils.translation import gettext_lazy as _

from catchup.validators import validate_is_audio


class Episode(models.Model):
    class Status(models.TextChoices):
        PROCESSING = "P", _("Processing")
        READY = "R", _("Ready")

    name = models.CharField()
    slug = models.SlugField()
    uploaded_at = models.DateTimeField("uploaded at", auto_now_add=True)
    publish_at = models.DateTimeField("publish at", default=timezone.now)

    original_file = models.FileField(upload_to="episodes/%Y/%m/%d/originals/", null=True, default=None, validators=[validate_is_audio])

    status = models.CharField(
        max_length=1,
        choices=Status,
        default=Status.PROCESSING,
    )

    def __str__(self):
        return f'{self.name} ({Episode.Status(self.status).label})'

    @property
    def is_published(self):
        return (self.status == Episode.Status.READY.name) and (self.publish_at <= timezone.now())

class EpisodeTag(models.Model):
    episode = models.ForeignKey(Episode, on_delete=models.CASCADE)
    tag = models.CharField()

    def __str__(self):
        return self.tag

    class Meta:
        unique_together = ('episode', 'tag')
