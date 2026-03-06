from django.db import models
from django.utils.translation import gettext_lazy as _

class Episode(models.Model):
    class Status(models.TextChoices):
        PROCESSING = "P", _("Processing")
        READY = "R", _("Ready")

    name = models.CharField()
    slug = models.SlugField()
    uploaded_at = models.DateTimeField("uploaded at")

    status = models.CharField(
        max_length=1,
        choices=Status,
        default=Status.PROCESSING,
    )

    def __str__(self):
        return f'{self.name} ({Episode.Status(self.status).label})'

class EpisodeTag(models.Model):
    episode = models.ForeignKey(Episode, on_delete=models.CASCADE)
    tag = models.CharField()

    def __str__(self):
        return self.tag

    class Meta:
        unique_together = ('episode', 'tag')
