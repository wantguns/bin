###### Builder Image
FROM rust as builder
WORKDIR /app
COPY . .
# scratch does not have the mkdir binary, so we create a folder here
RUN mkdir -p empty_upload

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
COPY --from=builder /app/empty_upload upload
COPY ./client upload/client
COPY ./templates templates
COPY ./static static
COPY ./themes themes
COPY --from=builder /usr/local/cargo/bin/bin .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6162

EXPOSE 6162
CMD ["./bin"]
