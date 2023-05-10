# bevy-prefab
have you ever wanted to spawn a prefab in bevy

so did i

```rust

fn square(x: f32, color: Color) -> SpriteBundle {
    // return a 50 by 50 square spritebundle at y=0 with the specified x coord and color
}

// usual Bevy API
commands
    .spawn(square(-360.0, Color::PINK))
    .with_children(|parent| {
        parent.spawn(square(60.0, Color::VIOLET));
        parent.spawn(square(120.0, Color::PURPLE));
        parent
            .spawn(square(180.0, Color::BLUE))
            .with_children(|parent| {
                parent.spawn(square(60.0, Color::CYAN));
                parent.spawn(square(120.0, Color::AQUAMARINE));
            });
    });

// prefab API
let prefab = square(0.0, Color::RED)
    .child(square(60.0, Color::ORANGE_RED))
    .child(square(120.0, Color::ORANGE))
    .child(
        square(180.0, Color::YELLOW)
            .child(square(60.0, Color::YELLOW_GREEN))
            .child(square(120.0, Color::GREEN)),
    );

commands.spawn_prefab(prefab);
```

how is this useful, you may ask

sugar

as for other capabilities like making runtime custonmizable factories that spawn runtime prefabs (at runtime) i'll have to think about it
