#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    window.eval("console.log('Hello from Rust')").unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}