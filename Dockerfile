FROM rust:1 as builder
WORKDIR /app
COPY . .

RUN cargo install --path .
RUN cargo clean

RUN mkdir -p upload
COPY ./client upload/client
COPY ./templates templates
COPY ./static static

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6162

EXPOSE 6162
CMD ["bin"]
