from django.db import models
from django.utils.timezone import now
from django.utils.translation import gettext_lazy as _


class IngestPoint(models.Model):
    name = models.CharField()
    icecast_url = models.URLField()

    def __str__(self) -> str:
        return self.name

class Livestream(models.Model):
    STATUSES = {
        "P": _("Pending"),
        "L": _("Live"),
        "E": _("Ended"),
    }

    name = models.CharField()
    slug = models.SlugField()
    scheduled_start = models.DateTimeField("scheduled start")

    ingest_point = models.ForeignKey(IngestPoint, on_delete=models.SET_NULL, null=True, default=None, blank=True)

    started_at = models.DateTimeField("stream started at", null=True, default=None)
    ended_at = models.DateTimeField("stream ended at", null=True, default=None)

    @property
    def status(self) -> str:
        if not self.started_at and not self.ended_at:
            return 'P'
        elif self.started_at and not self.ended_at:
            return 'L'
        else:
            return 'E'

    @property
    def starts_today(self) -> bool:
        return self.scheduled_start.date() == now().date()

    @property
    def started_today(self) -> bool:
        if not self.started_at: return False
        return self.started_at.date() == now().date()

    @property
    def ended_today(self) -> bool:
        if not self.ended_at: return False
        return self.ended_at.date() == now().date()

    def __str__(self) -> str:
        return f"{self.name} ({Livestream.STATUSES[self.status]})"

class LivestreamTag(models.Model):
    livestream = models.ForeignKey(Livestream, on_delete=models.CASCADE)
    tag = models.CharField()

    def __str__(self) -> str:
        return self.tag

    class Meta:
        unique_together = ("livestream", "tag")
