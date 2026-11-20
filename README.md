<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

<img align="right" width="250" src="assets/logo/tilt_svg.svg" alt="Lune logo" />

<h1 align="center">Lune</h1>

<div align="center">
	<div>
		<a href="https://crates.io/crates/lune">
			<img src="https://img.shields.io/crates/v/lune.svg?label=Version" alt="Current Lune library version" />
		</a>
		<a href="https://github.com/lune-org/lune/actions">
			<img src="https://shields.io/endpoint?url=https://badges.readysetplay.io/workflow/lune-org/lune/ci.yaml" alt="CI status" />
		</a>
		<a href="https://github.com/lune-org/lune/actions">
			<img src="https://shields.io/endpoint?url=https://badges.readysetplay.io/workflow/lune-org/lune/release.yaml" alt="Release status" />
		</a>
		<a href="https://github.com/lune-org/lune/blob/main/LICENSE.txt">
			<img src="https://img.shields.io/github/license/lune-org/lune.svg?label=License&color=informational" alt="Lune license" />
		</a>
	</div>
</div>

<br/>

A standalone [Luau](https://luau-lang.org) runtime.

Write and run programs, similar to runtimes for other languages such as [Node](https://nodejs.org), [Deno](https://deno.land), [Bun](https://bun.sh), or [Luvit](https://luvit.io) for vanilla Lua.

Lune provides fully asynchronous APIs wherever possible, and is built in Rust 🦀 for speed, safety and correctness.

## Features

- 🌙 Strictly minimal but powerful interface that is easy to read and remember, just like Luau itself
- 🧰 Fully featured APIs for the filesystem, networking, stdio, all included in the small (~5mb zipped) executable
- 📚 World-class documentation, on the web _or_ directly in your editor, no network connection necessary
- 🏡 Familiar runtime environment for Roblox developers, with an included 1-to-1 task scheduler port
- ✏️ Optional built-in library for manipulating Roblox place & model files, and their instances

## Non-goals

- Making programs short and terse - proper autocomplete / intellisense make using Lune just as quick, and readability is important
- Running full Roblox games outside of Roblox - there is some compatibility, but Lune is meant for different purposes

## Where do I start?

Head over to the [Installation](https://lune-org.github.io/docs/getting-started/1-installation) page to get started using Lune!

## The carley-rblx fork

This fork is upstream's latest release plus five fixes that make `@lune/roblox` read and write instances the way Roblox Studio does, so a place edited headless matches the same edit made in Studio. Each fix is its own branch on one release commit, so it can go upstream on its own. When upstream takes one, the fork drops it. Pin it in `aftman.toml` as `lune = "carley-rblx/lune@0.11.1-carley.1"`.

| Branch | What it fixes |
|---|---|
| `users/spoukyii/basepart-position-through-cframe` | `BasePart.Position`, `Orientation` and `Rotation` read from and write through `CFrame`. Before, reading threw "missing default value", and a write never moved the part and was dropped on save. |
| `users/spoukyii/addtag-no-duplicates` | `AddTag` ignores a tag the instance already has. Before, `GetTags` listed it twice and both copies were saved. |
| `users/spoukyii/vector-full-precision` | A `Vector3` or `CFrame` written to an instance keeps its full single precision. Before, it was rounded to 1/65536 of a stud, so a size of 0.462 saved as 0.4620056. |
| `users/spoukyii/attribute-whole-numbers` | `SetAttribute` keeps whole numbers past 2^31 (3000000000 read back as -1294967296). This is upstream's pull request #427, unchanged. |
| `users/spoukyii/axis-angle-normalize` | `CFrame.fromAxisAngle` brings its axis to length 1. Before, an axis of length 2 gave a frame scaled by 2. |

Each branch adds a test beside upstream's tests that fails on the release and passes with the fix. `UPSTREAM.md` holds each one's pull request text. The `carley` branch merges all five and adds this section, `UPSTREAM.md` and `.github/workflows/carley-release.yaml`.

The fork's base is `34055ac` ("Version 0.11.1"), the commit the published 0.11.1 binaries were built from. Upstream's `v0.11.1` tag points one commit later, at unreleased reverse property migration. `lune --version` still prints the upstream version.

### Releasing

Push a tag named `v<upstream version>-carley.<n>` on `carley`. The `Carley release` workflow builds the six platform zips under upstream's names (`lune-<version>-windows-x86_64.zip` and so on) and publishes them on that tag's release, where aftman finds them. The `CI` workflow builds and tests every pushed branch.

### Rebasing onto a new upstream release

1. `git fetch upstream --tags`. Find the commit the release's binaries were built from: the release's "Version x.y.z" commit, which may differ from where its tag points.
2. Rebase each patch branch onto it: `git rebase --onto <release commit> 34055ac users/spoukyii/<branch>`. Delete a branch upstream has merged. Fix conflicts in the branch, not in `carley`.
3. Rebuild `carley`: `git checkout -B carley <release commit>`, `git merge --no-ff` each remaining branch, then cherry-pick the fork's own commits from the old `carley` (`git log --no-merges --oneline <old carley> -- README.md UPSTREAM.md .github/workflows/carley-release.yaml`). Update the base commit and pin named in this section.
4. Push the branches and `carley --force-with-lease`, wait for CI to pass on every platform, then tag `v<new version>-carley.1` and push the tag.
