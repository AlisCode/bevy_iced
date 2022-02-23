# bevy_iced

A WIP, unreleased integration of the [`Iced`] UI framework for the [`Bevy`] game engine

[`Iced`]: https://github.com/iced-rs/iced
[`Bevy`]: https://github.com/bevyengine/bevy

# TODO before 0.1

Blocked on Iced 0.4 release, of course
List of stuff that needs to be worked on before the release :

## Requires Support

* Investigate if possible to dialog with ECS world from Command
    * Probably return a custom Command type?
* Investigate subscriptions if we want to support that ?
* Investigate render to target (by Image handle)
* Support Overlay
* Handle Clipboard correctly
* Handle Window actions (e.g. to change cursor)

## Usability

* Re-export convenient types
* Do not depend on Iced fork (see render.rs)

## Windowing

* Handle scale factor correctly

## Cleanup

* Recall StagingBelt when presenting everything
* Cleanup all TODOs
* Cleanup on struct visibility: check that everything is in order
* Move components to their own file
* Document all public structures + deny_warn

## Bevy Integration

* Use Handle<Font> instead of Iced's FontHandle
* Use Handle<Image> instead of Iced's ImageHandle
* -> These require a custom bevy_iced Backend that mostly reuses the code from iced_wgpu's Backend
* Personalize Iced settings through a resource + spawn IcedRenderer in a (common) startup system

## Examples

* Port pokedex example
* Port tour example
* "Interacting with ECS world" example

# Repository

* Proper README
* Demo
* Badges
* CI?
