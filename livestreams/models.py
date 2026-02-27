from django.db import models
from django.utils.translation import gettext_lazy as _

class Livestream(models.Model):
    class Status(models.TextChoices):
        PENDING = "P", _("Pending")
        LIVE = "L", _("Live")
        ENDED = "E", _("Ended")

    name = models.CharField()
    slug = models.SlugField()
    scheduled_start = models.DateTimeField("scheduled start")

    status = models.CharField(
        max_length=1,
        choices=Status,
        default=Status.PENDING,
    )

    def __str__(self):
        return f'{self.name} ({Livestream.Status(self.status).label})'

class LivestreamTag(models.Model):
    livestream = models.ForeignKey(Livestream, on_delete=models.CASCADE)
    tag = models.CharField()

    def __str__(self):
        return self.tag

    class Meta:
        unique_together = ('livestream', 'tag')
