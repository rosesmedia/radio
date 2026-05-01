from django.contrib import admin

from livestreams.models import Livestream, LivestreamTag, IngestPoint


class LivestreamTagInline(admin.TabularInline[LivestreamTag, Livestream]):
    model = LivestreamTag
    extra = 1


class LivestreamAdmin(admin.ModelAdmin[Livestream]):
    prepopulated_fields = {"slug": ["name"]}
    fields = ["name", "slug", "scheduled_start", "ingest_point"]
    list_display = ["name", "scheduled_start", "ingest_point"]
    inlines = [LivestreamTagInline]

admin.site.register(Livestream, LivestreamAdmin)
admin.site.register(IngestPoint)
