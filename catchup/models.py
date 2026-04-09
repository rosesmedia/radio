from django.db import models
from django.urls import reverse
from django.utils import timezone
from django.utils.translation import gettext_lazy as _

from catchup import PROCESSED_FILE_PATH_TEMPLATE, utils
from catchup.validators import validate_is_audio


class Episode(models.Model):
    class Status(models.TextChoices):
        PROCESSING = "P", _("Processing")
        READY = "R", _("Ready")

    name = models.CharField()
    slug = models.SlugField()
    uploaded_at = models.DateTimeField("uploaded at", auto_now_add=True)
    publish_at = models.DateTimeField("publish at", default=timezone.now)

    original_file = models.FileField(
        upload_to="episodes/%Y/%m/%d/originals/",
        null=True,
        default=None,
        validators=[validate_is_audio],
    )
    processed_file = models.FileField(
        upload_to=PROCESSED_FILE_PATH_TEMPLATE, null=True, default=None
    )

    duration_secs = models.PositiveIntegerField(null=True)

    status = models.CharField(
        max_length=1,
        choices=Status,
        default=Status.PROCESSING,
    )

    def __str__(self) -> str:
        return f"{self.name} ({Episode.Status(self.status).label})"

    @property
    def duration(self) -> str | None:
        return utils.format_duration(self.duration_secs) if self.duration_secs else None

    @property
    def is_published(self) -> bool:
        return (self.status == Episode.Status.READY.name) and (
            self.publish_at <= timezone.now()
        )

    def get_absolute_url(self):
        return reverse("catchup:detail", kwargs={"slug": self.slug})


class EpisodeTag(models.Model):
    episode = models.ForeignKey(Episode, on_delete=models.CASCADE)
    tag = models.CharField()

    def __str__(self) -> str:
        return self.tag

    class Meta:
        unique_together = ("episode", "tag")
