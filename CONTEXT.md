# Context
## Program Structure
For v0.3.0, basically we have:

1. User runs `albumctl build`
2. Read `albumctl.toml` in the source directory, fail if it doesn't exist.
3. Read `.albumctl/build.toml` in the output directory; if it doesn't exist, just output an empty
	`previous_build` HashMap index.
4. Create an empty `current_build` HashMap index.
4. For each module in `modules`:
5. Push all fields, overwriting if existing, into the `metadata` HashMap. Basically, this allows
	child modules to inherit fields from its parent modules, or override it if wanted.
6. If there is a `modules` field, go back to (3). Else, continue.
7. There should be a `files` field here in the last module in the line; in code, it should be an
	enum that can be either `modules` or `files`. If it contains `files`, then this module is a
	leaf. Otherwise, it is a branch. We start building here. Build logic will be talked about
	elsewhere. Note that, we should NOT fail the entire program if a build fails.
8. Report the build outcome once it finishes, e.g. successes, fails, deleted stales, etc.
9. Done!

## Incremental Build Process
1. We should have `metadata` and `files`. `files` should be another enum which describes how a file
	should be treated, e.g. if it's a single audio file, a file that needs to be cut, an SACD or
	some other format. We call the 'thing' that contains `metadata` and `files` or some processed
	version a build unit. A build unit is basically then entire module lineage compressed into the
	two variables.
2. We take the hash of `metadata` and `files`. `metadata` is hashed regularly, while for `files`, we
	use mtime + size. We don't want to hash the actual contents since that would take a long time,
	especially for very large files. Maybe we could provide a `--clean` option to force rebuild
	everything.
3. If that hash does NOT exist in the `previous_build` index, it means that either this is a new
	unit, or an existing unit with a changed source. Either way, we build it. The per-unit build
	process is detailed elsewhere. Then we push this into `current_build`, and save it into
	`build.toml`. We want to save the index with every unit build success, so that the entire build
	process isn't taken hostage by a single fallible write at the very end, invalidating the entire
	(and possibly quite long) build.
4. After building ALL units, we finally delete stale directories. How I'm thinking of doing this is
	to load all paths of `current_build` into some kind of prefix tree. Then we recurse through the
	output directory, deleting directories if it doesn't match a prefix in the corresponding level
	of the prefix tree. This *should* delete all unused and stale directories, keeping only the
	built units intact. Of course, we should also keep the `.albumctl` directory. The point of this
	is to delete the directories of units that *almost* finished building to the point that the
	built files are there in the output directory, but failed to write to the build index. This was
	a massive annoyance in v0.2.0 which I want to fix.
5. Done!

## Per-Unit Build Process
Time to get to the annoying bits, because this part uses a lot of filesystem ops. Fun!

1. We should have the unit hash at this point. First we mkdir `.albumctl/{hash}`. This will be our
	per-unit build directory. If it already exists, we recreate it. This is to discard previously
	failed unit builds.
2. We COPY (not move, for obvious reasons) the `files` into the build directory.
3. We do the processing into a consistent format here. No processing yet except for renaming; the
	cutting/SACD ripping will be implemented in v0.4.0. The filenames should be:
	`{disc}.{track:02} {title}.{original_extension}`. The extension isn't important.
4. If conversion is specified, use ffmpeg. For v0.3.0, we unconditionally convert without checking
	if the files are already the correct format. This creates copies; the filenames should be
	`{disc}.{track:02} {title}.{original_extension}.{target_extension}`. The original copied files
	are deleted, then the new files are renamed to remove the `.{extension}` part.
5. If replaygain is specified, use rsgain. This is an in-place operation, so no special handling
	required.
6. Finally, we move the processed files into the appropriate directory in the output directory.
	For now, this will be `{artist} - ({year}) {album}/({release_year}) {catalog_number} [{media_type}, {audio_channels}, {provenance}]`.
	Templates that will allow to modify this will be implemented in v0.5.0.
7. Done!
