# Audio packs

`generated/` contains the original MIT-licensed default cues and their provenance.
Rebuild with `python3 tools/generate_sounds.py`; no external inputs are needed.

`csgo/` and `local/` are explicitly ignored by Git. Other audio directories are
trackable by default. Keep private recordings in one of those ignored directories.
Opt into a catalog using `OPEN_STRIKE_AUDIO_PACK=assets/audio/local/catalog.ron`.
Catalog paths are relative to `assets/`; IDs match `generated/catalog.ron`.
Missing IDs and missing files use generated defaults. An unreadable or malformed
catalog falls back to the complete default pack. Invalid audio encoding remains
an asset-loading error; use `tools/index_sounds.py` to validate your local WAVs.

This override is a development convenience, not a grant of redistribution rights.
