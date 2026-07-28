use std::time::{SystemTime, UNIX_EPOCH};

use bevy::prelude::*;

#[derive(Component, Default)]
struct FpsCounter {
    frame_count: f64,
    current_time: f64,
    previous_time: f64,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, hello_world)
        .add_systems(Startup, setup)
        .add_systems(Update, counter_fps)
        .run();
}

fn hello_world() {
    println!("hello, bevy.");
}

fn setup(mut commands: Commands) {
    commands.spawn_empty().insert(FpsCounter::default());
}

fn counter_fps(mut counters: Query<&mut FpsCounter>) {
    let mut counter = counters
        .single_mut()
        .expect("Expected exactly one FpsCounter entity");

    if counter.previous_time == 0.0 {
        counter.previous_time = counter.current_time;
    }

    let now = SystemTime::now();
    let unix_time_f32 = now.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

    counter.current_time = unix_time_f32;

    if counter.current_time - counter.previous_time >= 1.0 {
        println!(
            "FPS: {}",
            counter.frame_count / (counter.current_time - counter.previous_time)
        );
        counter.frame_count = 0.0;
        counter.previous_time = counter.current_time;
    } else {
        counter.frame_count += 1.0;
    }
}
