#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};
use discord_presence::Client;
use tauri::Manager;
use lazy_static::lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref DISCORD_CLIENT: Mutex<Client> = Mutex::new(Client::new(1156699732765310976));
}

#[tauri::command]
fn presence(playing: bool, title: &str, author: &str, artwork: &str) {
    let client = &DISCORD_CLIENT;
    let mut client = client.lock().unwrap();

    client.set_activity(|act| act.details(title).state(author).assets(|ass| ass.large_image(artwork))).unwrap();
}


fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    window.eval("console.log('Hello from Rust')").unwrap();

                    //get state from window
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![presence])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}