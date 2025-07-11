################################################################################
# Create a stage for building the application.

ARG RUST_VERSION=1.78
ARG APP_NAME=mustore
FROM rust:${RUST_VERSION}-slim-bookworm AS build
ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=templates,target=templates \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    set -e && \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /app/$APP_NAME

################################################################################
# Create a stage for running the application.
FROM debian:bookworm-slim AS final

RUN apt update && apt install -y ca-certificates curl

# Create a non-privileged user that the app will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser


WORKDIR /app
# Copy the executable from the "build" stage.
COPY --from=build /app/$APP_NAME /app/$APP_NAME
# FIXME: we should pass config dynamically, not built container with it
COPY config /app/config
COPY migrations /app/migrations

# Expose the port that the application listens on.
EXPOSE 8000

# What the container should run when it is started.
CMD ["/app/mustore"]
