FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --target x86_64-unknown-linux-gnu --path .
RUN cargo clean

FROM debian:buster-slim as runner
WORKDIR /app
RUN mkdir -p upload
COPY ./client upload/client
COPY ./templates templates
COPY ./static static
COPY ./themes themes
COPY --from=builder /usr/local/cargo/bin/bin .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6162

EXPOSE 6162
CMD ["./bin"]
