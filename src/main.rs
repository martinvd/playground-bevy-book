use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("Hello World!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Jack Jackson".to_string())));
    commands.spawn((Person, Name("Joe Joeson".to_string())));
    commands.spawn((Person, Name("John Johnson".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0)
    }
}
