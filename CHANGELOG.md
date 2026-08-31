# Changelog

Since we follow [Conventional
Commits](https://decisions.seedcase-project.org/why-conventional-commits/),
we're able to automatically create formal releases based on the commit messages.
The releases are also published to `crate.io` for package distribution and
Zenodo for easier discovery, archival, and citation purposes. We use
[Cocogitto](https://decisions.seedcase-project.org/why-semantic-release-with-cocogitto/)
to be able to automatically create these releases, which uses
[SemVar](https://semverdoc.org) as the version numbering scheme, and [Git
Cliff](https://decisions.seedcase-project.org/why-changelog-with-git-cliff/) to
generate the changelog based on the commit messages.

Because releases are created based on commit messages, a new release is created
quite often---sometimes several times in a day. This also means that any
individual release will not have many changes within it. Below is a list of the
releases we've made so far, along with what was changed within each release.

Commits from bots, like `dependabot` or `pre-commit-ci`, are not included in the
changelog.

## [0.3.0](https://github.com/seedcase-project/seedcase-propagate/compare/0.2.0..0.3.0) - 2026-08-31

### ✨ Features

- Add structs for CLI commands
  [#105](https://github.com/seedcase-project/seedcase-propagate/pull/105) by
  [`@DanMazJen`](https://github.com/DanMazJen)
  ([d8058b7](https://github.com/seedcase-project/seedcase-propagate/commit/d8058b713a19a05aa8d5428a334926856a8431c0))

### 📝 Documentation

- Add TUI guide
  [#123](https://github.com/seedcase-project/seedcase-propagate/pull/123) by
  [`@joelostblom`](https://github.com/joelostblom)
  ([8806ecb](https://github.com/seedcase-project/seedcase-propagate/commit/8806ecbbfbb31ac185440f7e6f4793bd299d7dc7))

### 👩‍💻 Miscellaneous

- Ignore `CHANGELOG.md` for typos
  [#134](https://github.com/seedcase-project/seedcase-propagate/pull/134) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([ac99194](https://github.com/seedcase-project/seedcase-propagate/commit/ac99194fb1054a356893f6dbc8358241d798423b))

## [0.2.0](https://github.com/seedcase-project/seedcase-propagate/compare/0.1.0..0.2.0) - 2026-08-28

### ✨ Features

- Add request subset types
  [#85](https://github.com/seedcase-project/seedcase-propagate/pull/85) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([91a60ca](https://github.com/seedcase-project/seedcase-propagate/commit/91a60cad776fd1bde8eeb3279dc489d1fa3c42f2))

## [0.1.0] - 2026-08-28

### ✨ Features

- Add struct for the metadata of the data package
  [#59](https://github.com/seedcase-project/seedcase-propagate/pull/59) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([a4b89ca](https://github.com/seedcase-project/seedcase-propagate/commit/a4b89ca4a7bb3c1d38b9800976b8364854e57539))
- Add cli skeleton for `--help`
  [#73](https://github.com/seedcase-project/seedcase-propagate/pull/73) by
  [`@joelostblom`](https://github.com/joelostblom)
  ([ca91222](https://github.com/seedcase-project/seedcase-propagate/commit/ca91222a79b43f20f34612eb17e5c26bf6874f56))
- Add structs for the request information
  [#57](https://github.com/seedcase-project/seedcase-propagate/pull/57) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([d3d4d65](https://github.com/seedcase-project/seedcase-propagate/commit/d3d4d65f8aa88fd9f7690f72a0bedd18bfb9384e))

### 📝 Documentation

- Add Quarto website files by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([8eb4587](https://github.com/seedcase-project/seedcase-propagate/commit/8eb4587fdf682289508ea6faf12ab19dfd921a58))
- Add README and associated files by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([4a47ee0](https://github.com/seedcase-project/seedcase-propagate/commit/4a47ee0f55b4491179bdad208430e85f3575ba95))
- Add community health files by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([fcae054](https://github.com/seedcase-project/seedcase-propagate/commit/fcae05434f249076f9e467992c22c4cceb4d20ba))
- Empty CHANGELOG file by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([ed35b0f](https://github.com/seedcase-project/seedcase-propagate/commit/ed35b0f43efd23fdc1cf73800b36fdd56ba8f3f2))
- Add empty design doc files by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([1c5adf1](https://github.com/seedcase-project/seedcase-propagate/commit/1c5adf111322206528a8659c0975e9811d618513))
- Add design overview page
  [#20](https://github.com/seedcase-project/seedcase-propagate/pull/20) by
  [`@martonvago`](https://github.com/martonvago)
  ([16d6c51](https://github.com/seedcase-project/seedcase-propagate/commit/16d6c51986545b0a991793528fe0f0e390393ff9))
- Add design for input/output interface
  [#25](https://github.com/seedcase-project/seedcase-propagate/pull/25) by
  [`@signekb`](https://github.com/signekb)
  ([1327765](https://github.com/seedcase-project/seedcase-propagate/commit/13277650ba4d64495c868d5128db75c8597f86ff))
- Design for the CLI
  [#22](https://github.com/seedcase-project/seedcase-propagate/pull/22) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([2162666](https://github.com/seedcase-project/seedcase-propagate/commit/2162666b17aa144599dc5f9d4a58584bea00985d))
- Add architecture design page
  [#23](https://github.com/seedcase-project/seedcase-propagate/pull/23) by
  [`@martonvago`](https://github.com/martonvago)
  ([afcda6b](https://github.com/seedcase-project/seedcase-propagate/commit/afcda6bed879404344f169bc14025c4633e72b92))
- Reformat Markdown and minor edits
  [#35](https://github.com/seedcase-project/seedcase-propagate/pull/35) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([b56cb92](https://github.com/seedcase-project/seedcase-propagate/commit/b56cb925a418ebc3046bfa364bbd4a68b0eb3d5a))
- Remove empty spaces in badges, uncomment OpenSSF
  [#34](https://github.com/seedcase-project/seedcase-propagate/pull/34) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([6e1f6ec](https://github.com/seedcase-project/seedcase-propagate/commit/6e1f6ec33a3d4f8c7961c49a8eaf07380c3c851c))
- Add design for web application
  [#29](https://github.com/seedcase-project/seedcase-propagate/pull/29) by
  [`@signekb`](https://github.com/signekb)
  ([a67996f](https://github.com/seedcase-project/seedcase-propagate/commit/a67996f888203662a420aa728e943ae543be2c39))
- Add feature of selecting all columns in a resource in web app
  [#50](https://github.com/seedcase-project/seedcase-propagate/pull/50) by
  [`@signekb`](https://github.com/signekb)
  ([869bafc](https://github.com/seedcase-project/seedcase-propagate/commit/869bafcf2a702441e9bd7998e45ccf466575b8c7))
- Add landing and overview pages
  [#19](https://github.com/seedcase-project/seedcase-propagate/pull/19) by
  [`@DanMazJen`](https://github.com/DanMazJen)
  ([7924b95](https://github.com/seedcase-project/seedcase-propagate/commit/7924b9571e64bc614380df36ac9e042c2b24ae7a))
- Clarify that there should be one request per project
  [#51](https://github.com/seedcase-project/seedcase-propagate/pull/51) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([9b3c146](https://github.com/seedcase-project/seedcase-propagate/commit/9b3c14661110f538ae7f314b23708b18d54698a6))
- Render everything (`*`) but the `target/` folder
  [#67](https://github.com/seedcase-project/seedcase-propagate/pull/67) by
  [`@signekb`](https://github.com/signekb)
  ([58b35f8](https://github.com/seedcase-project/seedcase-propagate/commit/58b35f84988b14dffd2ddcf8db7ff96e49506dee))
- Expands on and describes user types in more detail
  [#56](https://github.com/seedcase-project/seedcase-propagate/pull/56) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([9ac30a7](https://github.com/seedcase-project/seedcase-propagate/commit/9ac30a753f9720d83b5feabc5a317ecf870452ed))
- Standardize on columns and rows
  [#71](https://github.com/seedcase-project/seedcase-propagate/pull/71) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([b14eb0b](https://github.com/seedcase-project/seedcase-propagate/commit/b14eb0b7a891779069fe9c3d2b63e31efc16a48d))
- Shorten headers in cli design
  [#72](https://github.com/seedcase-project/seedcase-propagate/pull/72) by
  [`@signekb`](https://github.com/signekb)
  ([8ebcd79](https://github.com/seedcase-project/seedcase-propagate/commit/8ebcd794e76d3d13b9e651fb68535d41fe1dc89e))
- Clarify purpose of TUI
  [#53](https://github.com/seedcase-project/seedcase-propagate/pull/53) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([835b37c](https://github.com/seedcase-project/seedcase-propagate/commit/835b37ceaddacc2006d76ed72888b7c0fff2723f))
- Add making a web app guide
  [#69](https://github.com/seedcase-project/seedcase-propagate/pull/69) by
  [`@martonvago`](https://github.com/martonvago)
  ([eb0412f](https://github.com/seedcase-project/seedcase-propagate/commit/eb0412f7a4900d04277864783fb6f3355e661b82))
- Add design docs for the request file
  [#65](https://github.com/seedcase-project/seedcase-propagate/pull/65) by
  [`@joelostblom`](https://github.com/joelostblom)
  ([ba19a47](https://github.com/seedcase-project/seedcase-propagate/commit/ba19a47cacfaee865a040a2be99dc0feb1227724))
- Add creating a subset guide
  [#70](https://github.com/seedcase-project/seedcase-propagate/pull/70) by
  [`@signekb`](https://github.com/signekb)
  ([5736474](https://github.com/seedcase-project/seedcase-propagate/commit/5736474104d16568681b70cb6ec01bf8f9230f5b))
- Clarify requirement for technical and non-technical interfaces
  [#87](https://github.com/seedcase-project/seedcase-propagate/pull/87) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([bffc250](https://github.com/seedcase-project/seedcase-propagate/commit/bffc250c2e1189b5070901bb24d07144b39f7222))
- Add guide listing
  [#92](https://github.com/seedcase-project/seedcase-propagate/pull/92) by
  [`@signekb`](https://github.com/signekb)
  ([6e02188](https://github.com/seedcase-project/seedcase-propagate/commit/6e021882badf3903c89c885ac8816967d7429c12))
- Update tagline
  [#91](https://github.com/seedcase-project/seedcase-propagate/pull/91) by
  [`@signekb`](https://github.com/signekb)
  ([955a730](https://github.com/seedcase-project/seedcase-propagate/commit/955a730ea483babbb91db62a6b78959e8471f642))
- Minor edits to improve the architecture page
  [#86](https://github.com/seedcase-project/seedcase-propagate/pull/86) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([54d5aaa](https://github.com/seedcase-project/seedcase-propagate/commit/54d5aaa2787ef4cb835f1c90d058617f70214528))
- Add installation guide
  [#82](https://github.com/seedcase-project/seedcase-propagate/pull/82) by
  [`@DanMazJen`](https://github.com/DanMazJen)
  ([c9c298a](https://github.com/seedcase-project/seedcase-propagate/commit/c9c298a5d2f074e2b71371d311da9ccce4952892))
- Describe reasons for web app and WASM
  [#68](https://github.com/seedcase-project/seedcase-propagate/pull/68) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([7404150](https://github.com/seedcase-project/seedcase-propagate/commit/7404150c0e0c9aa81096da3d648f4c20c0556a05))
- Draft TUI design
  [#101](https://github.com/seedcase-project/seedcase-propagate/pull/101) by
  [`@joelostblom`](https://github.com/joelostblom)
  ([d888861](https://github.com/seedcase-project/seedcase-propagate/commit/d888861274e19320a92fa4af5660f282e9d3a1c2))
- Add design for `check()`
  [#100](https://github.com/seedcase-project/seedcase-propagate/pull/100) by
  [`@signekb`](https://github.com/signekb)
  ([b86a0e8](https://github.com/seedcase-project/seedcase-propagate/commit/b86a0e88b2661fbe7e79bb28ed4fc6e67e2a7fa7))
- Clarify that `request.yaml` should be where `subset/` is
  [#90](https://github.com/seedcase-project/seedcase-propagate/pull/90) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([3ece7ab](https://github.com/seedcase-project/seedcase-propagate/commit/3ece7ab0e947beee17cf17396f1971c151dfba48))
- Add missing metadata to guide pages
  [#124](https://github.com/seedcase-project/seedcase-propagate/pull/124) by
  [`@joelostblom`](https://github.com/joelostblom)
  ([cd38742](https://github.com/seedcase-project/seedcase-propagate/commit/cd387424af570f2e001660551cac10f1f82116eb))
- Clarify subsets as sample populations with inclusion criteria
  [#104](https://github.com/seedcase-project/seedcase-propagate/pull/104) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([6a0b64a](https://github.com/seedcase-project/seedcase-propagate/commit/6a0b64af608e7aa1ea0b1a295cccc82387009eca))
- Add design for read functions
  [#106](https://github.com/seedcase-project/seedcase-propagate/pull/106) by
  [`@signekb`](https://github.com/signekb)
  ([63524b0](https://github.com/seedcase-project/seedcase-propagate/commit/63524b001d024ecd9534b20a0539630610120de0))

### 💄 Styling

- Add Quarto theme extension by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([4da332a](https://github.com/seedcase-project/seedcase-propagate/commit/4da332a02f52a227dbfbfe02509eef552663b9cb))
- Ran Markdown formatter
  [#89](https://github.com/seedcase-project/seedcase-propagate/pull/89) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([bac8c19](https://github.com/seedcase-project/seedcase-propagate/commit/bac8c191c65e87b4b3e77a0c8f641d2e5f63ff55))
- Fix line lengths
  [#103](https://github.com/seedcase-project/seedcase-propagate/pull/103) by
  [`@signekb`](https://github.com/signekb)
  ([f66078f](https://github.com/seedcase-project/seedcase-propagate/commit/f66078f08cc785d32b334e5a277be2b5924feb7b))
- Format `tui.qmd`
  [#107](https://github.com/seedcase-project/seedcase-propagate/pull/107) by
  [`@signekb`](https://github.com/signekb)
  ([9737771](https://github.com/seedcase-project/seedcase-propagate/commit/973777180330de01ae0ae51fb447bc20b8571327))

### 👷 CI/CD

- Add CI/CD workflows by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([03bcc3f](https://github.com/seedcase-project/seedcase-propagate/commit/03bcc3fe2bfc271ab481fcaf8a39ee09c7ab8713))
- Forgot to add file for Netlify by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([e1cbf0f](https://github.com/seedcase-project/seedcase-propagate/commit/e1cbf0fe600324eae9eb117111e70e33a12b7320))
- Ignore merge commits in commit check
  [#131](https://github.com/seedcase-project/seedcase-propagate/pull/131) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([9c77bdf](https://github.com/seedcase-project/seedcase-propagate/commit/9c77bdff08e753e4ced54975eb1220be1d0b2455))
- Install `cargo-edit` to bump `Cargo.toml` version
  [#132](https://github.com/seedcase-project/seedcase-propagate/pull/132) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([cfc56c9](https://github.com/seedcase-project/seedcase-propagate/commit/cfc56c9992e0502b97f9ccbbf9500aa52ec0c417))

### 👩‍💻 Miscellaneous

- Add `vscode` setting files by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([cbfdf1f](https://github.com/seedcase-project/seedcase-propagate/commit/cbfdf1f417dc9dfb547e3910c00cbc9f6d70a9b9))
- Add basic Rust package files by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([2804474](https://github.com/seedcase-project/seedcase-propagate/commit/28044748fd7644de3cda961ddad217ee720ac28a))
- Add justfile by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([ec669b4](https://github.com/seedcase-project/seedcase-propagate/commit/ec669b42de81e85b5e93af6fdbe792b249e71e32))
- Add config files by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([c8ef633](https://github.com/seedcase-project/seedcase-propagate/commit/c8ef6330a73e6ba9c9d0df690918e4085ddda8a4))
- Add basic DevEx files (CODEOWNERS, etc) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([2d62270](https://github.com/seedcase-project/seedcase-propagate/commit/2d62270e9085b0c62942ab511fb8418e5891339a))
- Add Netlify ID by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([9384664](https://github.com/seedcase-project/seedcase-propagate/commit/9384664d4bb17411be609c1c2287ef730d71d2fb))
- Upgrade Seedcase Quarto theme
  [#26](https://github.com/seedcase-project/seedcase-propagate/pull/26) by
  [`@signekb`](https://github.com/signekb)
  ([276862e](https://github.com/seedcase-project/seedcase-propagate/commit/276862ed99527c05c6def131c0c47ff96e2d5ca4))
- Correct to `.md`, not `.qmd` for changelog
  [#33](https://github.com/seedcase-project/seedcase-propagate/pull/33) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([8c6d207](https://github.com/seedcase-project/seedcase-propagate/commit/8c6d20741d320cd9f83a8101a076f4b7b01e97ac))
- Set correct typos config location (`.config/`)
  [#30](https://github.com/seedcase-project/seedcase-propagate/pull/30) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([4d59609](https://github.com/seedcase-project/seedcase-propagate/commit/4d59609feb79e01c406fd4c9641d9575d22c5487))
- Ignore `pre-commit.ci` and `github.com` checks, they are often blocked
  [#31](https://github.com/seedcase-project/seedcase-propagate/pull/31) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([e611dd8](https://github.com/seedcase-project/seedcase-propagate/commit/e611dd801234f9da1ef05625e7a3614e31963d1b))
- Allow "dirty" (changes in working dir) for cargo fixes
  [#32](https://github.com/seedcase-project/seedcase-propagate/pull/32) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([d45e510](https://github.com/seedcase-project/seedcase-propagate/commit/d45e51007c72af38f31217559146eb3a83fdf9f0))
- Ignore `_badges.qmd` when using rumdl, it mangles it
  [#54](https://github.com/seedcase-project/seedcase-propagate/pull/54) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([c1d2627](https://github.com/seedcase-project/seedcase-propagate/commit/c1d2627f56bf919481531f0705894a0ddab0e5b7))
- Remove leftover recipe code that is for the template
  [#55](https://github.com/seedcase-project/seedcase-propagate/pull/55) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([ad40351](https://github.com/seedcase-project/seedcase-propagate/commit/ad403513159d8ee472d5a1cc152489ff64ba88e8))
- Don't build docs of dependencies
  [#62](https://github.com/seedcase-project/seedcase-propagate/pull/62) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([9647c52](https://github.com/seedcase-project/seedcase-propagate/commit/9647c5217dfe4de5e91c28ca7f4baeaaaf3b710b))
- Add `rustfmt.toml` to wrap comments
  [#61](https://github.com/seedcase-project/seedcase-propagate/pull/61) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([33ab76e](https://github.com/seedcase-project/seedcase-propagate/commit/33ab76ebcff0936e8c325c817da38bf1dd16e56d))
- Set up structure of files in `src/`
  [#58](https://github.com/seedcase-project/seedcase-propagate/pull/58) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([e2b522d](https://github.com/seedcase-project/seedcase-propagate/commit/e2b522d74f47d39ab6c0ef9919b88820d18eb6a6))
- Add path to `rustfmt.toml` for rust-analyzer
  [#63](https://github.com/seedcase-project/seedcase-propagate/pull/63) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([65570fd](https://github.com/seedcase-project/seedcase-propagate/commit/65570fd120ebd902a719786ab8fce6f6c526cc9d))
- Ignore Markdown files in `target/` (Rust build folder)
  [#60](https://github.com/seedcase-project/seedcase-propagate/pull/60) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([3d546fd](https://github.com/seedcase-project/seedcase-propagate/commit/3d546fd1d1b8e433a29ac5bdc442053a47b3a9db))
- Update from Copier template
  [#83](https://github.com/seedcase-project/seedcase-propagate/pull/83) by
  [`@signekb`](https://github.com/signekb)
  ([dca66a4](https://github.com/seedcase-project/seedcase-propagate/commit/dca66a4f981353e3692e66449a9762401b97af7a))
- Correct path to `requests.qmd` file in Quarto sidebar
  [#88](https://github.com/seedcase-project/seedcase-propagate/pull/88) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([42c5cfb](https://github.com/seedcase-project/seedcase-propagate/commit/42c5cfb09dae5bd2fce97b7217e238a4c5189a23))

### ❤️ New contributors

- `@github-actions[bot]` started making automated contributions

- [`@lwjohnst86`](https://github.com/lwjohnst86) made their first contribution
  in [#132](https://github.com/seedcase-project/seedcase-propagate/pull/132)

- [`@signekb`](https://github.com/signekb) made their first contribution in
  [#106](https://github.com/seedcase-project/seedcase-propagate/pull/106)

- [`@joelostblom`](https://github.com/joelostblom) made their first contribution
  in [#124](https://github.com/seedcase-project/seedcase-propagate/pull/124)

- [`@DanMazJen`](https://github.com/DanMazJen) made their first contribution in
  [#82](https://github.com/seedcase-project/seedcase-propagate/pull/82)

- [`@martonvago`](https://github.com/martonvago) made their first contribution
  in [#69](https://github.com/seedcase-project/seedcase-propagate/pull/69)

- `@dependabot[bot]` started making automated contributions
