import magic
from django.core.exceptions import ValidationError
from django.db.models.fields.files import FieldFile
from django.utils.translation import gettext_lazy as _

ALLOWED_AUDIO_TYPES = [
    "audio/aac",
    "audio/mpeg",
    "audio/ogg",
    "audio/wav",
    "audio/webm",
    "audio/x-wav",
]


def validate_is_audio(file: FieldFile) -> None:
    m = magic.from_buffer(file.read(2048), mime=True)
    if m not in ALLOWED_AUDIO_TYPES:
        raise ValidationError(_("Must upload an audio file!"))
