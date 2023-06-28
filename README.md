# Scripty (v2)

rewrite of the [old Scripty](https://github.com/tazz4843/scripty) in general

# selfhosting

should work

linux support only, if you're on windows or mac, it might work, might not,
not gonna fix it or accept PRs relating to support

build
```shell
git clone https://github.com/scripty-bot/scripty
cd scripty
cargo build --release
```
download models from https://github.com/ggerganov/whisper.cpp

edit ``config.example.toml`` to your needs.
you must change at least:
* ``token``
* ``model_dir``
* ``token``
* ``database.*``

then run
```shell
RUST_LOG=debug ./target/release/scripty_v2 ./config.example.toml
```

it should start up

# docker? 
https://www.youtube.com/watch?v=PivpCKEiQOQ
