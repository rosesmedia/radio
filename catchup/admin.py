from django.contrib import admin

from catchup.models import EpisodeTag, Episode


class EpisodeTagInline(admin.TabularInline):
    model = EpisodeTag
    extra = 1

class EpisodeAdmin(admin.ModelAdmin):
    prepopulated_fields = {'slug': ['name']}

    fieldsets = [
        (None, {"fields": ["name", "slug", "publish_at"]}),
    ]

    list_display = ["__str__", "is_published"]

    inlines = [EpisodeTagInline]

admin.site.register(Episode, EpisodeAdmin)
