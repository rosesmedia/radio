from django.contrib import admin

from livestreams.models import Livestream, LivestreamTag

class LivestreamTagInline(admin.TabularInline[LivestreamTag, Livestream]):
    model = LivestreamTag
    extra = 1

class LivestreamAdmin(admin.ModelAdmin[Livestream]):
    prepopulated_fields = {'slug': ['name']}

    inlines = [LivestreamTagInline]

admin.site.register(Livestream, LivestreamAdmin)
