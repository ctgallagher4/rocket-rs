# Rocket

![Rocket](/assets/rocket.mp4)

Rocket is a simpler version of the hit game asteroids. Its purpose is to help people learn how to structure a Rust program in a fun way. Please feel free to reuse and/or modify Rocket however you like. This is not how you would structure an actual video game for best performance, but rather a simpler, object-oriented approach that is beginner friendly. For one way to structure a more advanced program, potentially with multi-threading, see [Entity Component System](https://en.wikipedia.org/wiki/Entity_component_system) on Wikipedia. For more information about the math involved, you can see my [website](https://ctgallagher4.github.io) under the blog section.

## Running Rocket

You will need to install SDL3 from source. Please see [this](https://github.com/libsdl-org/SDL/blob/main/INSTALL.md) for the latest instructions for your environment. After following those instructions and installing SDL3 with

```
make install
```

I needed to:

```bash
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib
```

After that, you can run Rocket with:

```bash
cargo run --release
```

A window should immediately pop up after compilation.

## Controls
 * w - accelerate forward
 * a - turn counterclockwise
 * d - turn clockwise
 * spacebar - fire a missile
