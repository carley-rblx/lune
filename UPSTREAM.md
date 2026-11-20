# Upstream pull requests

The text for each fix's pull request to `lune-org/lune`, ready to open from its branch in `carley-rblx/lune`. None is opened yet: that waits for the owner's word. Each branch is one commit on `34055ac` ("Version 0.11.1"), so rebase it onto upstream `main` before opening, and add its changelog line under `## Unreleased`, `### Fixed`, which the fork leaves out so the branches merge cleanly.

## Position, Orientation and Rotation on BasePart

Branch `users/spoukyii/basepart-position-through-cframe`. Title: `Read and write BasePart Position, Orientation and Rotation through CFrame`.

Reading `Position`, `Orientation` or `Rotation` on a part threw `Failed to get property 'Position' - missing default value`. Writing one did not error, but the value was stored as a separate property that never moved the part and was dropped on save, so code that works in Studio silently did nothing. Roblox saves only `CFrame` and treats these three as views of it, and they now read from it and write through it for every `BasePart`.

This reopens what #78 asked for, scoped to the three views of one property that Roblox itself never saves. Other classes with a `Position` (such as `GuiObject`) are untouched, and `tests/roblox/instance/classes/BasePart.luau` covers reading, writing, the round trip through `serializeModel`, and a `Frame`.

Changelog: `- Fixed `Position`, `Orientation` and `Rotation` on a `BasePart` in `@lune/roblox` not being readable, and writes to them being lost, by reading and writing them through `CFrame``

## AddTag on a tag the instance already has

Branch `users/spoukyii/addtag-no-duplicates`. Title: `Ignore AddTag for a tag the instance already has`.

Calling `AddTag` twice with the same tag stored it twice, so `GetTags` listed it twice and `serializeModel` saved both copies. Roblox keeps a tag once however often it is added, and `AddTag` now does the same. `tests/roblox/instance/tags.luau` checks the count before and after a round trip.

Changelog: `- Fixed `AddTag` in `@lune/roblox` adding a tag the instance already has a second time`

## Full precision for vectors written to instances

Branch `users/spoukyii/vector-full-precision`. Title: `Keep full precision for Vector3 and CFrame values written to instances`.

Every `Vector3` and `CFrame` written to an instance was rounded to the nearest 1/65536 of a stud, so a part sized 0.462 read back and saved as 0.4620056 while Roblox keeps 0.462 as a single-precision float. That made a script that runs twice over a place, or compares Lune's output with Studio's, see changes that were never made. The rounding was added to keep XML files tidy, but it changes the values themselves, so this removes it. It is a likely cause of the unchanged instances that show as changed in #108.

`tests/roblox/instance/properties.luau` checks a size, a position and a rotation before and after `serializeModel`.

Changelog: `- Fixed `Vector3` and `CFrame` values written to instances in `@lune/roblox` being rounded to the nearest 1/65536`

## Whole-number attributes past 2^31

Branch `users/spoukyii/attribute-whole-numbers`. This is #427 by chiliec, taken unchanged, so there is nothing to open: say on #427 that it fixes this too, and drop the branch from the fork once it merges.

## CFrame.fromAxisAngle with an axis not of length 1

Branch `users/spoukyii/axis-angle-normalize`. Title: `Normalize the axis given to CFrame.fromAxisAngle`.

`CFrame.fromAxisAngle(Vector3.new(0, 2, 0), math.pi / 2)` gave a frame scaled by 2, with a LookVector of (-2, 0, 0), because the axis was passed through as given. Roblox brings the axis to length 1 first and returns (-1, 0, 0), and Lune now does the same. `tests/roblox/datatypes/CFrame.luau` checks a unit axis and two that are not.

Changelog: `- Fixed `CFrame.fromAxisAngle` in `@lune/roblox` scaling the result when the axis is not of length 1`
