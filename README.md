# Scripty (v2)

rewrite of the [old Scripty](https://github.com/tazz4843/scripty) in general

## why?
thank [valk](https://github.com/randomairborne) for bugging me about it 🙃

# progress

The core features are mainly done, biggest issue right now is a segfault
when trying to run a model (source of it in coqui STT code, but our source is
[in speaking_update.rs](https://github.com/scripty-bot/scripty-rewrite/blob/9857f7a4f94ec95ff3d452f5789fca26bbbae3fc/scripty_audio_handler/src/events/speaking_update.rs#L50))

# selfhosting

should work

linux support only, if you're on windows or mac, it might work, might not,
not gonna fix it or accept PRs relating to support

get native Coqui STT libs
```shell
curl -L https://github.com/coqui-ai/STT/releases/download/v1.2.0/libstt.tflite.Linux.zip \
 | sudo busybox unzip - -d /lib -n
```

build
```shell
git clone https://github.com/scripty-bot/scripty-rewrite
cd scripty-rewrite
cargo build --release
```
download a model: https://coqui.ai/models

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