# selfhosting scripty via podman

## notes

Scripty's pre-built binaries only support x86_64 Linux:
if you're on another system you must [build from source](./selfhosting-from-source.md)

the podman build is completely untested

the binary runs but i have not tested if it's actually functional,
so expect broken stuff sometimes

official support for broken stuff is available, poke me on scripty's discord

rootful docker will never be officially supported, but it'll likely work anyway.
you should not be using rootful though because it's a massive security hole,
and if you ask for help while using rootful i'll tell you to use rootless or podman

rootless is essentially equivalent to podman, just substitute `docker` for `podman` in commands

## the image

this repo automatically builds a podman image every commit at
https://github.com/scripty-bot/scripty/pkgs/container/scripty

pull it with

```bash
podman pull ghcr.io/scripty-bot/scripty:master
```

## running

it expects `config.toml` mounted at `/app/config.toml`,
and scripty's internal webserver binds on `127.0.0.1:42069` by default.
change as necessary

a postgres database must be accessible somehow, configure that yourself

```bash
podman run -p 42069:42069 -v "./config.toml:/app/config.toml:ro" ghcr.io/scripty-bot/scripty:master
```
