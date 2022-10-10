# Deploy

Package the application as a container:

Put the output of

```bash
openssl rand -base64 32
```

into `ROCKET_SECRET_KEY` on `docker-compose.yml`.

change the credential environments and then

```bash
docker compose up -d
```
