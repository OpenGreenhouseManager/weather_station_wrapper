## Weather Station Wrapper — Containerization & Deployment

This service wraps a websocket API behind a REST API that caches the newest value.

### Build the Docker image

```bash
docker build -t weather-station-wrapper:local .
```

### Configuration

The binary expects the path to a JSON config file as its first CLI argument. The container defaults to `/config/config.json`.

Use `config/example.config.json` as a starting point:

```json
{
  "mode": "InputOutput",
  "port": 6001,
  "datasource_id": "7a224a14-6e07-45a3-91da-b7584a5731c1",
  "input_type": "Number",
  "output_type": "Number",
  "additional_config": {
    "base_url": "http://localhost:8080/",
    "application_key": "REPLACE_WITH_APPLICATION_KEY",
    "api_key": "REPLACE_WITH_API_KEY"
  },
  "scripting_api": null
}
```

Place your final config at `./config/config.json` (host), which will be mounted read-only into the container.

### Run with Docker

```bash
mkdir -p config
cp config/example.config.json config/config.json # edit values

docker run -d \
  --name weather-station-wrapper \
  --restart unless-stopped \
  -p 6001:6001 \
  -v "$(pwd)/config:/config:ro" \
  ghcr.io/OWNER/weather_station_wrapper:latest \
  /config/config.json
```

Notes:
- The app runs as a non-root user inside the container.
- The default port is 6001; change it via your config file.

### Run with Docker Compose

Edit `docker-compose.yaml` as needed (replace `OWNER` with your org/user), then run:

```bash
docker compose up -d
```

### GitHub Container Registry (GHCR) publishing

The workflow at `.github/workflows/publish.yml` builds and publishes multi-arch images to GHCR on pushes to `main` and on semver tags (`vX.Y.Z`).

Image names:
- `ghcr.io/<OWNER>/weather_station_wrapper:latest` (default branch)
- `ghcr.io/<OWNER>/weather_station_wrapper:<tag>` for releases
- `ghcr.io/<OWNER>/weather_station_wrapper:sha-<shortsha>` for every commit

To release:
1. Push to `main` to update `latest`, or
2. Create a tag like `v0.1.0` and push it.

### Security hardening

- Non-root user in runtime image.
- Minimal Debian slim with `ca-certificates` only.
- Mount configuration as read-only volume.
- Compose file sets `read_only: true` and `no-new-privileges`.


