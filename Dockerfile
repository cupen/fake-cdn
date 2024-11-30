FROM rust:1-alpine AS builder

RUN apk add --no-cache build-base openssl-dev

WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN touch src/main.rs #
RUN cargo build --release


FROM alpine:latest

RUN apk add --no-cache ca-certificates openssl
COPY --from=builder /workspace/target/release/fake-cdn /usr/bin/fake-cdn

WORKDIR /app
COPY conf ./conf
RUN addgroup -S fake-cdn && adduser -S fake-cdn -G fake-cdn
RUN mkdir .uploads && chown -R fake-cdn:fake-cdn /app
USER fake-cdn

EXPOSE 9527

ENV FAKECDN_CONFIG=/app/conf/conf.toml
ENV FAKECDN_DIR=/app/.uploads

CMD ["/usr/local/bin/fake-cdn", "web"] 