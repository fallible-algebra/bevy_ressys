use bevy::{prelude::*, log::LogPlugin};
use bevy_ressys::res_system;

fn main() {
    App::new()
        .add_plugin(LogPlugin::default())
        .add_startup_system(info_system)
        .add_startup_system(warn_system)
        .add_startup_system(warn_system2)
        .add_startup_system(error_system)
        .add_startup_system(one::two::three::error_system)
        .run();
}

#[derive(Debug, Component)]
struct A;

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

#[res_system(bevy::log::warn)]
fn warn_system3(mut q: Query<Entity, With<A>>) -> Result<(), String> {
    Ok(())
}

#[res_system(error)]
fn error_system(_commands: Commands) -> Result<(), String> {
    Err(format!("This is a error system"))
}

pub mod one {
    pub mod two {
        pub mod three {
            use bevy_ressys::res_system;
            use bevy::prelude::*;
            #[res_system(error)]
            pub fn error_system(_commands: Commands) -> Result<(), String> {
                Err(format!("This is a error system in a bunch of modules."))
            }
        }
    }
}