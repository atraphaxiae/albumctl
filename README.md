# `albumctl`

A declarative local music library manager.

`albumctl` generates a consistent music library from a canonical source. This source consists of the
configuration for the music library, the manifest files which describe your music, and the music
files themselves. Unlike traditional music library managers, `albumctl` never discovers or edits
metadata. All metadata are explicitly authored in manifest files. This design gives `albumctl` the
following properties:

- **Authority**. Manifest files are the single source of truth for your music metadata. `albumctl`
	does not rely on an external database like MusicBrainz.

- **Immutability**. The canonical source is never mutated by `albumctl`.

- **Reproducibility**. If the generated library is ever lost, it can always be regenerated with
	`albumctl`, provided that the canonical source is unchanged.

- **Declarativity**. The manifests and configuration describe what your music library should look
	like. `albumctl` takes care of organizing directories, naming files, and generating the library
	from that description.

`albumctl` revolves around four kinds of files:

- `albumctl.toml` is the configuration for the generated music library. Right now, this only
	contains its location.

- `album.toml` describes a single album. This contains the album's title, artist, original release
	year, etc.

- `release.toml` describes a single release of an album. This contains the release's release year,
	catalog number, media type, track listing, etc.

- `mapping.toml` describes how the tracks of a release map to the actual music files on disk. This
	allows `albumctl` to avoid relying on filename conventions or some other unreliable heuristic.

`albumctl` enforces a strict layout on the source directory. Each subdirectory in the source
directory should be an album directory, and each subdirectory in an album directory should be a
release directory. For example:

- `src/`
	- `albumctl.toml`
	- `Wayne Shorter - (1966) Speak No Evil/`
		- `album.toml`
		- `2015 Music Matters 33RPM Vinyl Rip/`
			- `release.toml`
			- `mapping.toml`
			- `01 - Witch Hunt.flac`
			- *music files and auxiliary files here...*

The directory names don't matter. `albumctl` identifies albums and releases from the manifest files,
not from the names of the directories. The only requirement is that the source directory follows
this hierarchy.

Then, just by running `albumctl build`, `albumctl` generates your music library from the source,
producing a clean, reproducible music library at the configured output directory, while keeping
the source unchanged. This generated library is disposable; if it is ever lost, it can always be
recreated using `albumctl build`.
