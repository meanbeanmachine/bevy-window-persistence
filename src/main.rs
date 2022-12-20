use std::{fs::{File, create_dir_all}, io::{Read, Write}};
use bevy::{prelude::*, window::WindowCloseRequested};
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

fn main() {
    // Load previous window position from saved config file, if it exists
    let last_window_position = load_config();

    // Build App
    App::new()
    .add_plugin(WindowPlugin {
        close_when_requested: false,
        window: WindowDescriptor {
            position: last_window_position.position,
            // monitor: last_window_position.monitor,
            ..default()
        },
        ..default()
    })
    .add_plugins(
        DefaultPlugins
        .build()
        .disable::<WindowPlugin>()
    )
    .add_startup_system(print_position)
    .add_system(window_close)
    .run();
}

// Window position when the app is closed; used to restore window position on boot
#[derive(Debug, Serialize, Deserialize)]
struct LastWindowPosition {
    position: WindowPosition,
    // monitor: MonitorSelection
}

// Load Config File
fn load_config() -> LastWindowPosition {
    // Setup Load Path
    // Lin: /home/alice/.config/barapp
    // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
    // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    let dir = ProjectDirs::from("com", "meanbeanmachine",  "window").expect("ProjectDirs").config_dir().to_owned();
    let path = dir.join("config.ron").to_owned();

    // load
    let file = File::open(path);
    // return default values if file does not exist
    if file.is_err() { return LastWindowPosition { position: WindowPosition::Centered }; }
    // if file.is_err() { return LastWindowPosition { position: WindowPosition::Centered, monitor: MonitorSelection::Current }; }
    
    // buffer
    let mut buffer = String::new();
    file.unwrap().read_to_string(&mut buffer).expect("buffer config file");

    // parse
    return ron::from_str::<LastWindowPosition>(&buffer).expect("parse RON");
}

// Print the Saved Position on Startup
fn print_position(windows: Res<Windows>){
    let boot_position = windows.primary().position().expect("print_position: get window position");
    warn!("On Boot Position: {:?}", boot_position);
}

// Custom Window Close Handler
fn window_close (event: EventReader<WindowCloseRequested>, mut windows: ResMut<Windows>){
    if event.len() == 0 { return };
    event.clear();

    // Fetch primary window since this is a one-window app
    let window = windows.primary_mut();

    // Save Window's Position
    save_window_config(&window);

    // Close App
    window.close();
}

fn save_window_config(window: &Window) {
    // Fetch Window Position
    let window_position = window.position().expect("save_window_config: get window position");
    warn!("On Exit Position: {:?}", window_position);

    // todo!(Fetch Monitor Index)
    // let monitor_index = window.monitor()??????

    // Update Config File
    let last_window_position = LastWindowPosition { position: WindowPosition::At(Vec2::new(window_position.x as f32, window_position.y as f32))};
    //todo!(Ideal when fetching Monitor Index is actually possible)
    // let last_window_position = LastWindowPosition { position: WindowPosition::At(Vec2::new(window_position.x as f32, window_position.y as f32)), monitor: MonitorSelection::Index(???) };

    // Save
    save_file(last_window_position);
}

fn save_file(last_pos: LastWindowPosition){
    // Setup Save Path
    // Lin: /home/alice/.config/barapp
    // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
    // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    let dir = ProjectDirs::from("com", "meanbeanmachine",  "window").expect("ProjectDirs").config_dir().to_owned();
    let path = dir.join("config.ron").to_owned();

    // Create ron data to save
    let data = ron::to_string::<LastWindowPosition>(&last_pos).expect("ron to string");
    
    // Save
    create_dir_all(dir).expect("create save dir");
    File::create(path).and_then(|mut file| file.write(data.as_bytes())).expect("save config file");
}