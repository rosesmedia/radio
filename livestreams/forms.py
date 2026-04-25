from django import forms
from django.core.exceptions import ValidationError
from django.utils.translation import gettext as _

from livestreams import stream_controller


def validate_source(value: str) -> None:
    if value not in stream_controller.SOURCES.values():
        raise ValidationError(_("Invalid source: %s") % value)

class EmptyForm(forms.Form):
    pass

class SourceForm(forms.Form):
    source = forms.CharField(validators=[validate_source])
