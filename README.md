# i-cant-believe-its-not-bsn

Ergonomic ways to spawn Bevy entity hierarchies.

Eagerly [waiting for BSN](https://github.com/bevyengine/bevy/discussions/14437)? Really wish you could spawn hierarchies with less boilerplate? Just want to define some reusable widget types for `bevy_ui` in code?

This crate is here to help!

## Helper Components

You can use the helper component `WithChild`, and its iterator sibling, `WithChildren`, to embed hierarchy information in normal bundles.

Just add it as a component holding the bundle you want to use to spawn the child, and you're off to the races. A component hook will see that this component has been added, extract the data from your `WithChild` component, and then move it into a child, cleaning itself up as it goes.

These helper components are extremely useful when you just want to insert a tree of entities declaratively.

### In Required Components

Also, these components can be used to spawn children entities with required components:

```rust
use bevy::prelude::*;
use i_cant_believe_its_not_bsn::WithChild;

#[derive(Component)]
#[require(Transform, WithChild<PlayerHead>, WithChild<PlayerBody>)]
struct Player;

#[derive(Component, Default)]
struct PlayerHead;

#[derive(Component, Default)]
struct PlayerBody;

fn main() {
    let mut app: App = App::new();
    app.add_systems(Startup, spawn_player);
    app.update();

    let world: &mut World = app.world_mut();
    let player_entity: Entity = world
        .query_filtered::<Entity, With<Player>>()
        .single(world)
        .unwrap();

    let children: &Children = world.get::<Children>(player_entity).unwrap();
    assert_eq!(children.len(), 2);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player);
}
```

## The Template Macro

Alternatively, you can use the `template!()` macro, which is very similar to the proposed BSN syntax. Arbitrary data can be passed into the macro using normal rust blocks. It returns portable `Template` values, which can be spliced into other templates using `@{ ... }`.

Not only is this macro declarative and composable - it also supports basic incrementalization (doing partial updates to the ECS rather than rebuilding from scratch)! Building the same macro multiple times with `commands.build(template)` does the minimal amount of work necessary to bring the ECS into alignment with the template.

## Bevy Version

Different versions of this crate support different versions of Bevy Engine:

| Crate Version | Bevy Version |
| ------------- | ------------ |
| 0.1           | 0.14         |
| 0.2, 0.3      | 0.15         |
| 0.4           | 0.16.1       |
