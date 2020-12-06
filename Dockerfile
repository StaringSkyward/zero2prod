# Plan stage
FROM ekidd/rust-musl-builder:stable as planner
WORKDIR /app
# To ensure a reproducible build consider pinning
# the cargo-chef version with–version X.X.X‘
RUN cargo install cargo-chef
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json


# Cach stage
FROM ekidd/rust-musl-builder:stable AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json /app/
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json


# Build stage
FROM ekidd/rust-musl-builder:stable AS builder
WORKDIR /app
# Copy over the cached dependencies
COPY --from=cacher /app/target /app/
COPY . .
ENV SQLX_OFFLINE true
# Build our application, leveraging the cached deps!
RUN cargo build --release --bin zero2prod


# Runtime stage
FROM alpine:latest AS runtime
RUN addgroup -S zero2prod && adduser -S -G zero2prod zero2prod
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/zero2prod /app/
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]