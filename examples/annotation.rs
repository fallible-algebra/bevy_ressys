use bevy::{prelude::*, log::LogPlugin};
use bevy_ressys::res_system;

fn main() {
    App::new()
        .add_plugin(LogPlugin::default())
        .add_startup_system(info_system)
        .add_startup_system(warn_system)
        .add_startup_system(warn_system2)
        .add_startup_system(error_system)
        .run();
}

#[res_system(info)]
fn info_system(_commands: Commands) -> Result<(), String> {
    Err(format!("This is a info system"))
}

#[res_system(warn)]
fn warn_system(_commands: Commands) -> Result<(), String> {
    Err(format!("This is a warning system"))
}

#[res_system(bevy::log::warn)]
fn warn_system2(_commands: Commands) -> Result<(), String> {
    Err(format!("This is another warning system, showing you can use full macro paths"))
}

#[res_system(error)]
fn error_system(_commands: Commands) -> Result<(), String> {
    Err(format!("This is a error system"))
}