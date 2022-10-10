# backend
FROM rust:1.64.0-buster AS backend
WORKDIR /app
RUN rustup toolchain install nightly && rustup default nightly
COPY ./backend ./
RUN cargo build --release

# frontend
FROM node:16-buster AS frontend
WORKDIR /app
COPY ["./frontend/package.json", "./frontend/package-lock.json*", "./"]
RUN npm install --omit=dev
COPY ./frontend .
RUN npm run build

# join together
FROM debian:buster-slim AS production
WORKDIR /app/backend
ENV LOG_FILE=/var/log/samba/audit.log
ENV LOGIN_USER=admin
ENV LOGIN_PASSWORD=admin
ENV API_KEY=test
ENV ROCKET_PORT=80
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_SECRET_KEY=

COPY --from=frontend /app/build /app/frontend/build
COPY --from=backend /app/ .

# RUN chmod +x samba-manager
ENTRYPOINT [ "./target/release/samba-manager" ]