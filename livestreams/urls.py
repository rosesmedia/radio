from django.urls import path

from livestreams import control_views

app_name = "livestreams"

urlpatterns = [
    path("control/", control_views.IndexView.as_view(), name="control_index"),
    path("control/<str:slug>/", control_views.StreamControlView.as_view(), name="control_stream"),
]
