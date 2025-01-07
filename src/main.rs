use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(TestPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(), MainCamera));
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource, Default)]
pub struct CursorPosition(Vec2);

pub struct TestPlugin;

impl TestPlugin {
    pub fn hello_world() {
        println!("Hello world");
    }

    pub fn cursor_system(
        q_window: Query<&Window, With<PrimaryWindow>>,
        mut cursor_pos: ResMut<CursorPosition>,
    ) {
        let window = q_window.single();

        if let Some(pos) = window.cursor_position() {
            cursor_pos.0 = pos;
            println!("Position: {}", pos);
        }
    }
}

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, TestPlugin::hello_world);
        app.add_systems(Update, TestPlugin::cursor_system);
        app.insert_resource(CursorPosition::default());
    }
}
