# Install docker

```sh
curl -fsSL get.docker.com -o get-docker.sh && sh get-docker.sh
```

# Configure samba

Example
/etc/samba/smb.conf

```conf
[share]
vfs objects = full_audit
full_audit:prefix =[SMB AUDIT] %u|%I|%m|%S
full_audit:success = create_file connect disconnect open read renameat write unlinkat translate_name mkdirat
full_audit:facility = LOCAL5
full_audit:priority = notice
```

/etc/rsyslog.d/samba.conf

```r
$template format, "%timestamp:::date-rfc3339%%msg%\n"

local5.* /var/log/samba/audit.log;format
& stop
```

audit.log permissions to avoid problems

```
touch /var/log/samba/audit.log
chmod 660 /var/log/samba/audit.log
chown syslog:adm /var/log/samba/audit.log
```

# Deploy

Package the application as a container:

```bash
openssl rand -base64 32
```

Put the output of above into `ROCKET_SECRET_KEY` on `docker-compose.yml`.

```bash
docker compose up -d
```
