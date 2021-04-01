FROM rust:1.50 as builder

RUN mkdir /app
WORKDIR /app
ADD . /app

RUN cargo build --release

FROM debian:buster-slim

LABEL org.opencontainers.image.source=https://github.com/heyvito/gh-auth

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=ghauth

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER

COPY --from=builder /app/target/release/gh-auth /gh-auth

RUN chown -R $APP_USER:$APP_USER /gh-auth

USER $APP_USER
WORKDIR /

CMD ["/gh-auth"]