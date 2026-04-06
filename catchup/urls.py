from django.urls import path

from . import views

app_name = "catchup"

urlpatterns = [
    path("", views.RecentEpisodesView.as_view(), name="index"),
    path("<str:slug>/", views.EpisodeView.as_view(), name="detail"),
]
