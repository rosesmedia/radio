from django.contrib import admin

from livestreams.models import Livestream

class LivestreamAdmin(admin.ModelAdmin):
    prepopulated_fields = {'slug': ['name']}

admin.site.register(Livestream, LivestreamAdmin)
