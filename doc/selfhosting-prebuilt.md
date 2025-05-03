# selfhosting scripty - prebuilt executable

only supports linux, will not support any other OS, especially not windows

WSL does not count as linux, use a VM at the very least (although good luck getting GPU passthrough)

Scripty's pre-built binaries only support x86_64 Linux using glibc:
if you're on another system you must [build from source](./selfhosting-from-source.md)

## clone the repo

some files in the repo are required for scripty to run
(mostly internationalization files)

```bash
git clone https://github.com/scripty-bot/scripty
cd scripty
```

## download the binary

each commit builds a binary

1. head to the github web UI and
   click the action icon directly to the right of the latest commit title
2. click "Details" to the right of "Build executable / Build binary"
3. click summary in the upper left
4. scroll down to artifacts
5. hit the download button on `scripty_v2-x86_64-unknown-linux-gnu`
6. save to a convenient location (most likely directly in the root directory of the repo)

## running

continue from the "migrate database" section of the [build from source page](./selfhosting-from-source.md),
and skip the "build" section
