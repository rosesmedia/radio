from django.urls import path

from . import views

app_name = "roses2026"

urlpatterns = [
    path("", views.index, name="index"),
]
