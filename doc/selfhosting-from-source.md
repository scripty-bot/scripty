# selfhosting - building from source

only supports linux, will not support any other OS, especially not windows

WSL does not count as linux, use a VM at the very least (although good luck getting GPU passthrough)

## requirements

* rust: https://rustup.rs/ (nightly)
* postgresql: https://www.postgresql.org/download/ (tested with 17.4)
* valkey: https://valkey.io/ (tested with 8.1.0)
* mold linker https://github.com/rui314/mold
* clang (for mold)
* pkg-config (libopus dependency)
* libopus
* stt-service: https://github.com/scripty-bot/stt-service (latest, refer to its README for more info)

### recommended distros

* arch: has the easiest install process for the latest versions of everything, and is what
  scripty runs on in production
* debian: has old versions of stuff (that will likely still work) but is still a lot better than the
  dumpster fire that is ubuntu

#### arch

```shell
sudo pacman -S postgresql redis mold clang base-devel pkgconf
```

#### all distros

install rust with this (command from https://rustup.rs/)

make sure to pick rust nightly during the install process,
scripty liberally uses nightly features

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## prepare

### clone repo

```shell
git clone https://github.com/scripty-bot/scripty
cd scripty
```

### migrate database

```shell
cargo install sqlx-cli # skip if you already have it
sudo -u postgres createuser scripty -P # will prompt for password
sudo -u postgres createdb -O scripty scripty
cp .env.example .env
nano .env # edit DATABASE_URL to match your database
cargo sqlx migrate run
```

#### troubleshooting

double check `pg_hba.conf` and `postgresql.conf` if you get a connection error

## build

```shell
cargo build --release
```

## configure

```shell
cp config.toml.example config.toml
nano config.toml # edit to your liking
# at least one stt service must exist
```

## run

```shell
./target/release/scripty_v2
```

## advanced

### MUSL build

official support removed (kinda), why bother building with a libc that's a few megabytes smaller but much slower?
if you want to run on alpine linux anyway we'll still help you, it should be as simple as adding
`--target x86_64-unknown-linux-musl` if cross-compiling for target, otherwise just normal build for native

if it breaks poke me
