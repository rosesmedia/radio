# Guide for working on R.A.D.I.O.

## Service dependencies

All services needed for development can be started with Docker Compose (or Podman Compose):

- `docker compose pull`
- `docker compose up --detach`

### OvenMediaEngine

[OvenMediaEngine](https://docs.ovenmediaengine.com/) is used as the backend for the final streaming outputs.

## Django app

Dependencies are managed via [`uv`](https://docs.astral.sh/uv/):

- `uv sync`: Download all dependencies
- `uv run manage.py migrate`: Run database migrations
- `uv run manage.py runserver` Run the development server.

### Postgres

The main Django app uses Postgres as a database. Postgres connection settings can be found in `roses_radio/settings.py`.

### RabbitMQ

RabbitMQ is used as the message broker for [cellery](https://docs.celeryq.dev/en/stable/) background tasks.

To test background tasks, you will need to run a celery worker:

```bash
uv run celery -A roses_radio worker
```

## Livestreaming

Livestream processing is done using [liquidsoap](https://liquidsoap.info). You will need to copy the configuration from `liq/restreamer.example.conf` to `liq/restreamer.local.conf`. Audio files are stored in `git-lfs`, so if they were not included when you cloned the repo, ensure it is [installed and configured correctly](https://github.com/git-lfs/git-lfs#getting-started).
