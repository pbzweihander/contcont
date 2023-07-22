FROM rust:1.70.0-slim AS backend

WORKDIR /app

COPY backend backend
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

WORKDIR /app/backend

RUN cargo build --release

FROM node:20-slim AS frontend

WORKDIR /app

COPY frontend frontend

WORKDIR /app/frontend

RUN yarn && yarn build

FROM debian:stable-slim

RUN apt-get update &&\
    apt-get install -y ca-certificates &&\
    rm -rf /var/lib/apt/lists/*

COPY --from=backend /app/target/release/contcont /usr/local/bin/contcont
COPY --from=frontend /app/frontend/dist /srv/static

ENV STATIC_FILES_DIRECTORY_PATH=/srv/static

CMD ["contcont"]
