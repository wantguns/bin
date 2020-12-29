FROM rustlang/rust:nightly-slim AS builder
WORKDIR app
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt-get update -y \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/bin bin
RUN mkdir -p upload
COPY ./templates templates
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6162

ENTRYPOINT ["./bin"]
