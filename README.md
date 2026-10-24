# wuli

A 2D rigid-body physics engine in Rust, using Extended Position Based Dynamics (XPBD).

## Workspace

| Crate | What it is |
|---|---|
| [`wuli`](wuli) | The engine: bodies, collision detection and the XPBD solver |
| [`wuli-scenes`](wuli-scenes) | Ready-made worlds for the viewer and tests |
| [`wuli-client`](wuli-client) | A macroquad viewer |

## Usage

```bash
cargo run -p wuli-client
```

| Input | Action |
|---|---|
| <kbd>Space</kbd> | Pause / resume |
| <kbd>.</kbd> (hold) / <kbd>,</kbd> (tap) | Step while paused |
| <kbd>R</kbd> | Reset |
| Mouse wheel | Zoom |
| Right mouse drag | Pan |

```rust
use glam::{DVec2, dvec2};
use wuli::{RigidBody, World};

let mut world = World::new(-9.81);

// Inverse mass 0 makes a body static
world.add_body(RigidBody::circle(dvec2(0.0, -10.0), DVec2::ZERO, 0.0, 10.0));
let ball = world.add_body(RigidBody::circle(dvec2(0.0, 2.0), DVec2::ZERO, 1.0, 0.5));

world.step(1.0 / 60.0);
println!("{}", world.body(ball).unwrap().position);
```

## Resources

- [Extended Position Based Dynamics](https://www.mysimulator.uk/content/articles/xpbd-dynamics.html)

## License

[GPL-3.0](LICENSE)
