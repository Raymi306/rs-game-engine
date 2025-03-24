# rs-game-engine

A software rendering style game engine library with resource management and font drawing capability that is designed to allow for quickly spinning up a game or experiment.

The workflow centers around creating an object implementing the `GameState` trait, which defines the major lifecycle hooks for the engine:
- `on_create`
- `on_update`
- `on_exit`

This workflow is inspired by the simplicity of the [Pixel Game Engine](https://github.com/OneLoneCoder/olcPixelGameEngine)

Please check out the [examples](examples)!
In addition to showing off how to use various features, they also function as a set of tests to ensure everything is working as expected.
Some unit tests also exist in various parts of the library
