from django.urls import path

from . import views

app_name = "catchup"

urlpatterns = [
    path("<str:slug>/", views.EpisodeView.as_view(), name="detail"),
]
