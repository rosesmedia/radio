from django.contrib import admin

from livestreams.models import Livestream, LivestreamTag

class LivestreamTagInline(admin.TabularInline):
    model = LivestreamTag
    extra = 1

class LivestreamAdmin(admin.ModelAdmin):
    prepopulated_fields = {'slug': ['name']}

    inlines = [LivestreamTagInline]

admin.site.register(Livestream, LivestreamAdmin)
