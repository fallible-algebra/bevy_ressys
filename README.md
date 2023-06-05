# bevy_ressys

Annotation macro for writing systems that return results for logging for [Bevy](https://github.com/bevyengine/bevy).

This is an implementation of a potential fix for https://github.com/bevyengine/bevy/issues/8638, where piping systems that return errors to the logging util systems reports the location of the error as the util system and not the system that generated the error.

```rust
#[res_system(bevy::log::warn)]
fn this_system_warns(/* args */) -> Result<(), String> {
    Err("This is a warning".to_owned())
}
```

Becomes

```rust
fn this_system_warns(/* args */) {
    fn inner_fn_for_res_system(/* args */) {
        Err("This is a warning".to_owned())
    }
    if let Err(err) = inner_fn_for_res_system(/* args */) {
        bevy::log::warn!(err)
    }
}
```
