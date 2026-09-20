# `albumctl`
A declarative builder for your music library.

> [!WARNING]
> Due to how `albumctl` works, there is a risk of data loss. Do not use this program unless you
> understand and accept this risk.

`albumctl` builds your music library from a source directory of modules containing metadata and
source music files. Unlike other music library managers, `albumctl` does not edit or discover
metadata from outside sources. All metadata are user-provided via the modules.

Modules may either contain other child modules or source music files. Child modules inherit the
metadata of their ancestors, while the source music files are processed according to the metadata of
their containing module. An example of a source directory is:

```
music_library
├── albumctl.toml
└── Wayne Shorter - (1966) Speak No Evil
    ├── module.toml
    └── (2015) MMBST-84194
        ├── module.toml
        ├── Track 1.flac
        ├── Track 2.flac
        ├── Track 3.flac
        ├── Track 4.flac
        ├── Track 5.flac
        └── Track 6.flac
```

`music_library` is the root module, while `Wayne Shorter - (1966) Speak No Evil` is its child
module, and so on. `albumctl.toml` contains the configuration of the music library, while
`module.toml` files contain metadata.

Running `albumctl build <DIR>` will then build your music library from the source, outputting it in
the directory specified in `albumctl.toml`. If you ever lose this music library, you can always just
run the build again.

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

Ensure that Cargo's binary directory is in your `PATH` environment variable so that you can call
`albumctl` directly.

## Basic Usage
Create the directory where you want to store your source files, then create `albumctl.toml` there.
Say I want to have the album 'A Love Supreme' in my music library:

```toml
# albumctl.toml

output_directory = "/home/atraphaxiae/Music" # This must be a full path

[metadata]
# Add global metadata here

[children]
modules = [
	# This is a path to a folder containing a module.toml
	"John Coltrane - (1965) A Love Supreme"
]
```

Then, create the module for the album. In my case, I create the directory
`John Coltrane - (1965) A Love Supreme` and put a `module.toml` inside, with more metadata:

```toml
# John Coltrane - (1965) A Love Supreme/module.toml

[metadata]
artist = "John Coltrane"
album = "A Love Supreme"
year = "1965"

catalog_number = "A-77"
release_year = "1965"
media_type = "33 RPM"
audio_channels = "Stereo"
provenance = "PBTHAL"
```

Note that the metadata fields shown above are required for now; the leaf module (which defines the
file mapping) must have those fields, either inherited from its ancestors or defined in its own
metadata table.

I also want to define a tracklist here:

```toml
# John Coltrane - (1965) A Love Supreme/module.toml

# ...

# This puts us in Disc 1
[[discs]]
[discs.metadata]
# You can add per-disc metadata here

# This puts us in Track 1.01
[[discs.tracks]]
[discs.tracks.metadata]
# You can add per-track metadata here
title = "Part I - Acknowledgement"

# This puts us in Track 1.02
[[discs.tracks]]
[discs.tracks.metadata]
title = "Part II - Resolution"

# This puts us in Disc 2
[[discs]]
[discs.metadata]

# This puts us in Track 2.01
[[discs.tracks]]
[discs.tracks.metadata]
title = "Part III - Pursuance - IV - Psalm"
```

A leaf module must also have a tracklist, again either inherited from its ancestors or defined in
itself. For now, each track must have a `title` field in its metadata table.

Then, I can define the file mapping here too:

```toml
# John Coltrane - (1965) A Love Supreme/module.toml

# ...

[children]
[[children.files]]
file = "01 - Acknowledgement.flac" # This is a path relative to this module

# This maps the file to Track 1.01 in the tracklist
disc_number = 1
track_number = 1

[[children.files]]
file = "02 - Resolution.flac"
disc_number = 1
track_number = 2

[[children.files]]
file = "03 - Pursuance - Psalm.flac"
disc_number = 2
track_number = 1
```

There are no restrictions on the module hierarchy. You can make a module hierarchy as complex as you
want. This example only shows a `Source -> Release` hierarchy, but you can also conceivably have:

- `Source -> Album -> Release`
- `Source -> Artist -> Year -> Album -> Release Year -> Release`
- `Source -> Genre -> Release`

Or whatever module hierarchy you want.

Now the file tree of the source directory looks like:

```
music-library-source
├── albumctl.toml
└── John Coltrane - (1965) A Love Supreme
    ├── module.toml
    ├── 01 - Acknowledgement.flac
    ├── 02 - Resolution.flac
    └── 03 - Pursuance - Psalm.flac
```

Then, running `albumctl build <DIR>`, where `DIR` is your source directory, `albumctl` will build
and output your music library in `output_directory`. In my case, I'll get the following output
directory tree:

```
/home/atraphaxiae/Music
└── John Coltrane - (1965) A Love Supreme
    └── (1965) A-77 [33 RPM, Stereo, PBTHAL]
        ├── 1.01 Part I - Acknowledgement.flac
        ├── 1.02 Part II - Resolution.flac
        └── 2.01 Part III - Pursuance - IV - Psalm.flac
```

Right now, `albumctl` outputs your music directory using an `Output -> Album -> Release` hierarchy.
In the future this will be customizable, along with which metadata to use for generating file and
folder names.

A more comprehensive example of how to use `albumctl` can be found in
my [`music-library`](https://github.com/atraphaxiae/music-library) repository, which I use to
generate my own music library.

## Documentation
The comprehensive documentation of `albumctl` is found at
[`https://atraphaxiae.github.io/albumctl/`](https://atraphaxiae.github.io/albumctl/).
