FROM rustlang/rust:nightly-alpine3.21 as build

WORKDIR /scripty

COPY . .

RUN apk add clang mold autoconf libtool automake make cmake

ENV RUSTFLAGS="--emit=asm --cfg tokio_unstable -Clink-arg=-fuse-ld=mold -Zshare-generics=y"
ENV SQLX_OFFLINE=1

# There's no libopus.a whatsoever in the alpine repositories, so we're forced to build from source
# Half the installed packages above are *just* for opus
ENV LIBOPUS_NO_PKG=1

RUN cargo build --release

FROM alpine:3.21

WORKDIR /app
COPY --from=build /scripty/target/release/scripty_v2 .

RUN adduser --home /nonexistent --no-create-home --disabled-password scripty
USER scripty

VOLUME "/app/config.toml"

CMD ["/app/scripty_v2", "/app/config.toml"]
