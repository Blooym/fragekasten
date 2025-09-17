# ----------
#   SETUP
# ----------
FROM alpine:latest AS setup
RUN adduser -S -s /bin/false -D fragekasten
RUN mkdir /dir

# -----------
#    BUILD
# -----------
FROM rust:1-alpine AS build
WORKDIR /build
RUN apk add --no-cache --update build-base

# Pre-cache dependencies
COPY ["Cargo.toml", "Cargo.lock", "./"]
RUN mkdir src \
    && echo "// Placeholder" > src/lib.rs \
    && cargo build --release \
    && rm src/lib.rs

# Build
ARG SQLX_OFFLINE true
COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx
COPY static ./static
COPY src ./src
RUN cargo build --release

# -----------
#   RUNTIME
# -----------
FROM scratch
WORKDIR /opt

COPY --from=build /build/target/release/fragekasten /usr/bin/fragekasten

# Setup deployment image.
COPY --from=setup /etc/passwd /etc/passwd
COPY --from=setup /bin/false /bin/false
USER fragekasten
COPY --from=setup --chown=fragekasten /dir /srv/fragekasten

# Set configuration defaults for container builds.
ENV FRAGEKASTEN_ADDRESS=0.0.0.0:6251
ENV DATABASE_URL=sqlite:///srv/fragekasten/data.db
ENV RUST_LOG=info
EXPOSE 6251

ENTRYPOINT ["/usr/bin/fragekasten"]