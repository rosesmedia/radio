import datetime
from django.urls import reverse
from django.utils import timezone
from django.test import TestCase

from catchup.models import Episode


def create_episode(title: str, slug: str, days: int) -> Episode:
    """
    Create an episode with the given `title`, `slug` and published the
    given number of `days` offset to now (negative for episodes published
    in the past, positive for episodes that have yet to be published).
    """
    time = timezone.now() + datetime.timedelta(days=days)
    return Episode.objects.create(name=title, slug=slug, publish_at=time)


class EpisodeDetailViewTests(TestCase):
    def test_future_episode(self) -> None:
        """
        The detail view of an episode with a publish_at in the future
        returns a 404 not found.
        """
        unpublished_episode = create_episode(title="Unpublished episode", slug='unpublished-episode', days=5)
        url = reverse("catchup:detail", args=(unpublished_episode.slug,))
        response = self.client.get(url)
        self.assertEqual(response.status_code, 404)

    def test_past_episode(self) -> None:
        """
        The detail view of an episode with a publish_at in the past
        displays the episode's url.
        """
        published_episode = create_episode(title="Published episode", slug='published-episode', days=-5)
        url = reverse("catchup:detail", args=(published_episode.slug,))
        response = self.client.get(url)
        self.assertContains(response, published_episode.name)

