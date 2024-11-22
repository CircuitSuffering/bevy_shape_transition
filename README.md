# bevy_shape_transition

## Description

This crate provides a way to transition between colors/screens in a Bevy game using shapes.

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
bevy_shape_transition = { git = "https://github.com/circuitsuffer-studios/bevy_shape_transition" }
```

## Examples

### Left Right

<img src="./assets/gifs/example_left_right.gif" width="50%" height="50%"/>

### Party

<img src="./assets/gifs/example_party.gif" width="50%" height="50%"/>

### Usage

Add the plugin:

```rust
use bevy::prelude::*;
use bevy_shape_transition::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TransitionPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}
```

Spawn a transition:

```rust
fn update(
    mut events: EventWriter<NewTransition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut color: Local<Color>
) {
    // color swap
    let color1 = Color::rgb(0.0, 0.0, 0.0);
    let color2 = Color::rgb(1.0, 1.0, 1.0);
    *color = if *color == color1 { color2 } else { color1 };

    // spawn transition
    if keyboard_input.just_pressed(KeyCode::Space) {
        events.send(NewTransition {
            angle: 0.0,
            color,
            duration: 1.8,
            ease: EaseFunction::QuarticIn,
        });
    }
}
```
