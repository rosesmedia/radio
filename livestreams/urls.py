from django.urls import path

from livestreams import control_views, api_views, views

app_name = "livestreams"

urlpatterns = [
    path("", views.LiveStreamsView.as_view(), name="index"),
    path("control/", control_views.IndexView.as_view(), name="control_index"),
    path("control/<str:slug>/", control_views.StreamControlView.as_view(), name="control_stream"),
    path("control/<str:slug>/start", control_views.start_stream, name="start_stream"),
    path("control/<str:slug>/stop", control_views.stop_stream, name="stop_stream"),
    path("control/<str:slug>/source", control_views.stream_source, name="source"),
    path("<str:slug>/", views.LivestreamView.as_view(), name="detail"),
    path("api/<str:slug>/config.json", api_views.stream_config_view, name='stream_config'),
]
