# `albumctl`

A declarative builder for your music library.

`albumctl` builds your music library from a source containing your albums, their releases, and their
manifest files. Unlike other music library managers, `albumctl` does not discover or edit metadata.
All metadata are provided by the user in the manifest files. There are four kinds of manifest files:

- `albumctl.toml` is the configuration for your music library. Currently, this only specifies the
	output directory for your music library.

- `album.toml` describes an album. This contains its title, artist, original release year, etc.

- `release.toml` describes a release of an album. This contains its release year, catalog number,
	media type, etc. as well as its tracklist.

`albumctl` enforces a strict layout on the source directory. The root directory must contain an
`albumctl.toml`, each subdirectory must be an album directory which contains an `album.toml`, and
each subdirectory of that must be a release directory which contains a `release.toml`. For example:

```
/src
├── albumctl.toml
└── Speak No Evil
    ├── album.toml
    └── MM33 Vinyl Rip
        ├── release.toml
        ├── 01 - Witch Hunt.flac
        └── ...
```

Running `albumctl build` will then build your music library at the output directory specified by
`albumctl.toml`, without modifying your source directory. If you ever lose your music library, you
can just rebuild it again.

## Installation
Install the latest release from [`crates.io`](https://crates.io/crates/albumctl):

```sh
cargo install albumctl
```

Or build the latest development version from source:

```sh
git clone https://github.com/atraphaxiae/albumctl.git
cd albumctl
cargo install --path .
```

Ensure that Cargo's binary directory is in your `PATH` so that you can call `albumctl` directly.
