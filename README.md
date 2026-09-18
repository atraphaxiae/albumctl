# `albumctl`
A declarative builder for your music library.

> [!WARNING]
> Due to how this `albumctl` works, there is a risk of data loss. Do not use this program unless you
> are understand and accept this risk.

`albumctl` builds your music library from a source directory of modules containing metadata and
source music files. Unlike other music library managers, `albumctl` does not edit or discover
metadata from outside sources. All metadata are user-provided via the modules.

Modules may either contain other child modules or source music files. Child modules inherit the
metadata of their parents, while the source music files are processed according to the metadata of
their containing module. An example of a source directory is:

```
music_library
├── albumctl.toml
└── John Coltrane - (1965) A Love Supreme
    ├── module.toml
    └── (2010) AIPJ 77
        ├── module.toml
        ├── Track 1.flac
        ├── Track 2.flac
        ├── Track 3.flac
        └── Track 4.flac
```

`music_library` is the root module, while `John Coltrane - (1965) A Love Supreme` is its child
module, and so on. `albumctl.toml` contains the configuration of the music library, while
`module.toml` files contain metadata.

Running `albumctl build <DIR>` will then build your music library from the source, outputting it in
the directory specified in `albumctl.toml`. If you ever lose this music library, you can always just
run the build again.

## Installation
Install the latest release from [crates.io](https://crates.io/crates/albumctl):

```sh
cargo install albumctl
```

Or build the latest development version from source:

```
git clone https://github.com/atraphaxiae/albumctl.git
cd albumctl
cargo install --path .
```

Ensure that Cargo's binary directory is in your `PATH` environment variable so that you can call
`albumctl` directly.

## Documentation
The comprehensive documentation of `albumctl` is found at [`https://atraphaxiae.github.io/albumctl/`](https://atraphaxiae.github.io/albumctl/).
