from django.contrib import admin
from django.contrib import messages
from django.db.models import QuerySet
from django.http import HttpRequest

from catchup import tasks
from catchup.models import EpisodeTag, Episode
from django.utils.translation import gettext_lazy as _
from django.utils.translation import ngettext_lazy as _n

class EpisodeTagInline(admin.TabularInline[EpisodeTag, Episode]):
    model = EpisodeTag
    extra = 1

class EpisodeAdmin(admin.ModelAdmin[Episode]):
    prepopulated_fields = {'slug': ['name']}

    fieldsets = [
        (None, {"fields": ["name", "slug", "publish_at"]}),
        (_("Files"), {"fields": ["original_file"]}),
    ]

    list_display = ["__str__", "is_published"]

    inlines = [EpisodeTagInline]

    actions = ['queue_upload_processing']

    @admin.action(description="Queue processing")
    def queue_upload_processing(self, request: HttpRequest, queryset: QuerySet[Episode]) -> None:
        count = 0
        for episode in queryset:
            tasks.process_upload.delay(episode.pk, True)
            count += 1
        self.message_user(
            request,
            _n(
                "%d episode was queued for processing.",
                "%d episodes were queued for processing.",
                count,
            ) % count,
            messages.SUCCESS
        )

admin.site.register(Episode, EpisodeAdmin)
