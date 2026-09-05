# License scope and asset provenance

The root MIT license applies to Open Strike's original code, documentation,
and procedural audio generator and its outputs in `assets/audio/generated/`.
It does not grant rights to third-party recordings, models, maps, textures,
animations, fonts or trademarks. Dependencies retain their respective licenses.

| Content | Current status |
| --- | --- |
| `assets/audio/generated/` | Original synthesis from seeded noise and oscillators; no sampled recordings. MIT; generator, parameters and output checksums are provided. |
| `assets/audio/csgo/` and `assets/audio/local/` | Private local content, excluded from Git and distribution. No redistribution permission is asserted. |
| `assets/fonts/` | Roboto Condensed; retain the bundled SIL Open Font License in `OFL.txt`. |
| Imported maps, character/weapon models, textures and animation sources | Third-party content. Existing attribution is recorded in README, but provenance and upstream rights have not been independently cleared for publication. |
| Generated GLBs, menu scene, map thumbnail and rifle icon | Derived from imported art. Generation, format conversion or modification does not make them MIT assets. They retain the underlying asset obligations. |

Before publishing the full game, verify the original source and license of each
third-party asset, including permission covering any underlying game content.
An attribution on a model-sharing page is not by itself confirmation that the
uploader owned those rights. Dust 2 and its derived menu scene require this review
alongside the character, rifle and animation sources. Replace or exclude content
whose distribution rights cannot be established.

Audio separation alone is not clearance to publish all assets currently present
in this repository. Use generated audio for public demonstrations unless the
recordings used have permission covering that use. Ignoring a local file does
not change its copyright status.
