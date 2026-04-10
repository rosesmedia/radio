from django.urls import path

from . import views

app_name = "catchup"

urlpatterns = [
    path("", views.RecentEpisodesView.as_view(), name="index"),
    path("<str:slug>/", views.EpisodeView.as_view(), name="detail"),
    path("add", views.create_episode, name='create'),
    path("<str:slug>/edit", views.UpdateEpisodeView.as_view(), name="update"),
    path("<str:slug>/edit/tags", views.update_episode_tags, name="update_tags"),
]
