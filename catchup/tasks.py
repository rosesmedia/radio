import tempfile
from celery import shared_task
from django.core.files import File

from catchup import ffmpeg_utils
from catchup.models import Episode


@shared_task
def process_upload(episode_pk: int, force: bool=False) -> bool:
    episode = Episode.objects.get(pk=episode_pk)
    if not force and episode.processed_file is not None:
        print(f'Episode {episode.pk} already has a processed file!')
        return False

    if not episode.original_file:
        print(f'Episode {episode.pk} is missing a source file!')
        return False

    input_info = ffmpeg_utils.ffprobe(episode.original_file.path)
    if 'format' not in input_info or 'duration' not in input_info['format']:
        print(f'Episode {episode.pk} has what seems to be an invalid file')
        return False

    duration = round(float(input_info['format']['duration']))
    episode.duration_secs = duration

    tmp = tempfile.NamedTemporaryFile(suffix='.flac')
    ffmpeg_utils.loudnorm(episode.original_file.path, tmp.name)
    episode.processed_file = File(tmp, name=f'{episode.pk}.flac')
    episode.status = Episode.Status.READY

    episode.save()
    tmp.close()

    return True
