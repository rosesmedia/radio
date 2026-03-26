from django.contrib import admin

from catchup.models import EpisodeTag, Episode
from django.utils.translation import gettext_lazy as _

class EpisodeTagInline(admin.TabularInline):
    model = EpisodeTag
    extra = 1

class EpisodeAdmin(admin.ModelAdmin):
    prepopulated_fields = {'slug': ['name']}

    fieldsets = [
        (None, {"fields": ["name", "slug", "publish_at"]}),
        (_("Files"), {"fields": ["original_file"]}),
    ]

    list_display = ["__str__", "is_published"]

    inlines = [EpisodeTagInline]

admin.site.register(Episode, EpisodeAdmin)
