# Build image
FROM rust:1.60 as builder

# Run dummy build to build and cache dependencies that only depends on Cargo.toml and Cargo.lock
WORKDIR /usr/src
RUN USER=root cargo new ostrich-api
COPY Cargo.toml Cargo.lock /usr/src/ostrich-api/
WORKDIR /usr/src/ostrich-api
RUN cargo build --release

# Run actual build
COPY ./src ./src
RUN cargo build --release --bin stripe_service

# Run image
FROM debian:bullseye-slim
RUN apt-get update
RUN apt-get -y install libpq-dev
RUN apt-get -y install libssl1.1
RUN apt-get install -y --no-install-recommends ca-certificates


# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder  /usr/src/ostrich-api/target/release/stripe_service /usr/local/bin/stripe-service

WORKDIR /usr/stripe-service
COPY ./.env ./.env
CMD ["stripe-service"]