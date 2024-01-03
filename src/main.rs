use bevy::prelude::*;

fn main() {
    App::new()
        // Load plugins
        .add_plugins((DefaultPlugins, HelloPlugin))
        // Run the app
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Silly Sausage".to_string())));
    commands.spawn((Person, Name("John Doe".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // Update timer and check if it finished
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resources
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            // Startup Systems
            .add_systems(Startup, add_people)
            // Update systems
            .add_systems(Update, greet_people);
    }
}
