# Scripty (v2)

rewrite of the [old Scripty](https://github.com/tazz4843/scripty) in general

# selfhosting

should work, untested

only supports linux, will not support any other OS, especially not windows

WSL does not count as linux, use a VM at the very least (although good luck getting GPU passthrough)

## requirements
* rust: https://rustup.rs/ (nightly)
* postgresql: https://www.postgresql.org/download/ (tested with 15.3)
* redis: https://redis.io/download (tested with 7.0.12)
* mold linker
* clang (for mold)
* pkg-config (libopus dependency)
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
nano .env # edit DATABASE_URL to match your URL
cargo sqlx migrate run
```
#### troubleshooting
double check `pg_hba.conf` and `postgresql.conf` if you get a connection error

## build
```shell
cargo build --release 
# or if you want a more optimized build
# takes about 3.5 minutes on a 13700K vs 45s for the normal build
# probably not worth it for casual users
cargo build --profile release-full
```

## configure
```shell
cp config.toml.example config.toml
nano config.toml # edit to your liking
# note stt
```

## run
```shell
./target/release/scripty_v2
```

note that serenity currently has a bug where ctrl+c does not work:
to shutdown, send it a ctrl+c to terminate all open connections, wait for a
warning about timeout elapsed, then run `pkill scripty` to kill the process

## advanced
### MUSL build
same as above, but build with `--target x86_64-unknown-linux-musl`
and execute `./target/x86_64-unknown-linux-musl/release/scripty_v2`

on arch also install the `musl` package

# docker? 
https://www.youtube.com/watch?v=PivpCKEiQOQ

# translations?

<a href="https://hosted.weblate.org/engage/scripty-bot/">
<img src="https://hosted.weblate.org/widgets/scripty-bot/-/open-graph.png" alt="Translation status" />
</a>
