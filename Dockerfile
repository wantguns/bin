###### Builder Image
FROM rust as builder
WORKDIR /app
COPY . .

ARG ARCH
RUN __ARCH="$(dpkg --print-architecture)"; \
    [ -z  $ARCH ] || __ARCH=$ARCH; \
    case "$__ARCH" in \
        arm64) \
            export __TARGET='aarch64-unknown-linux-gnu'; \
            apt update && apt upgrade -y; \
            apt install -y gcc-aarch64-linux-gnu; \
            rustup target add aarch64-unknown-linux-gnu; \
            ;; \
        amd64) export __TARGET='x86_64-unknown-linux-gnu' ;; \
    esac; \
    cargo install --target $__TARGET --path .;

RUN cargo clean


###### Runner Image
FROM scratch as runner
COPY --from=builder /usr/local/cargo/bin/bin .

ENV BIN_ADDRESS=0.0.0.0
EXPOSE 6162

CMD ["./bin"]
