# R.A.D.I.O.

Roses Audio Distribution Interface and Orchestration

The backend for Roses 2026 radio streaming.

## Architecture

The main application is a [django](https://www.djangoproject.com/) project, using PostgreSQL as a database.

Streaming will be handled using a combination of [liquidsoap](https://liquidsoap.info) and [mediamtx](https://mediamtx.org).

See [radio-tx](https://github.com/rosesmedia/radio-tx) (used at Roses 2025), which used a similar architecture.

## Development

A `docker-compose.yml` is provided that runs all services the app requires. The django app can be launched using `uv`:

```bash
uv run python3 manage.py runserver
```

Check the [django docs](https://docs.djangoproject.com/en/6.0/) for more information on working with django applications.

Issues are managed on [Linear](https://linear.app).
