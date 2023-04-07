// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    thread::{sleep, spawn},
    time::Duration,
};
use tauri::{
    command, generate_context, generate_handler, App, AppHandle, Builder, Event, EventHandler,
    Manager, Window,
};

#[derive(Debug, Serialize, Deserialize)]
struct MyMessage {
    name: String,
    age: u32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
fn my_custom_command() {
    println!("I was invoked from JS!");
}

#[command]
fn command_with_message(message: String) -> String {
    format!("hello {}", message)
}

#[command]
fn command_with_object(message: MyMessage) -> MyMessage {
    let MyMessage { name, age } = message;

    MyMessage {
        name: format!("hello {}", name),
        age,
    }
}

#[command]
fn command_with_error(arg: u32) -> Result<String, String> {
    if arg % 2 == 0 {
        Ok(format!("even value {}", arg))
    } else {
        Err(format!("odd value {}", arg))
    }
}

#[command]
async fn async_command() -> String {
    "hello".into()
}

fn open_dev_tools(app: &mut App) {
    #[cfg(debug_assertions)]
    {
        let window: Window = app.get_window("main").unwrap();
        window.open_devtools();
        // window.close_devtools();
    }
}

fn event_handler(app: &mut App) -> EventHandler {
    app.listen_global("front-to-back", |event: Event| {
        println!(
            "got front-to-back with payload {:?}",
            event.payload().unwrap()
        )
    })
}

fn ping_loop(app: &mut App) {
    let app_handle: AppHandle = app.app_handle();
    spawn(move || loop {
        app_handle
            .emit_all("back-to-front", "ping frontend".to_string())
            .unwrap_or_else(|_| ());
        sleep(Duration::from_secs(3))
    });
}

fn setup_entry(app: &mut App) -> Result<(), Box<dyn Error>> {
    {
        open_dev_tools(app);
        let _id = event_handler(app);
        ping_loop(app);
    }

    Ok(())
}

fn main() {
    Builder::default()
        .setup(setup_entry)
        .invoke_handler(generate_handler![
            greet,
            my_custom_command,
            command_with_message,
            command_with_object,
            command_with_error,
            async_command
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
