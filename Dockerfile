FROM rustlang/rust:nightly-bookworm as build

WORKDIR /scripty

COPY . .

RUN apt update -y
RUN apt install -y clang mold libopus-dev libopus0

ENV RUSTFLAGS="--emit=asm --cfg tokio_unstable -Clink-arg=-fuse-ld=mold -Zshare-generics=y"
ENV SQLX_OFFLINE=1


RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=build /usr/lib/x86_64-linux-gnu/libopus.so.0.8.0 /usr/lib/x86_64-linux-gnu/libopus.so.0.8.0
COPY --from=build /usr/lib/x86_64-linux-gnu/libopus.so.0 /usr/lib/x86_64-linux-gnu/libopus.so.0

WORKDIR /app
COPY --from=build /scripty/target/release/scripty_v2 .

RUN adduser --home /nonexistent --no-create-home --disabled-password scripty
USER scripty

VOLUME "/app/config.toml"

CMD ["/app/scripty_v2", "/app/config.toml"]
