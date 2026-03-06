from django.contrib import admin

from catchup.models import EpisodeTag, Episode


class EpisodeTagInline(admin.TabularInline):
    model = EpisodeTag
    extra = 1

class EpisodeAdmin(admin.ModelAdmin):
    prepopulated_fields = {'slug': ['name']}

    inlines = [EpisodeTagInline]

admin.site.register(Episode, EpisodeAdmin)
