#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};
use discord_presence::{Client, Event};
use tauri::Manager;
use lazy_static::lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref RPC: Mutex<Client> = Mutex::new(Client::new(1156699732765310976));
}

#[tauri::command]
fn presence(playing: bool, title: &str, author: &str, artwork: &str) {
    print!("presence");
    let mut client = RPC.lock().unwrap();

    if Client::is_ready() {
        print!("presence part 2");
        client.set_activity(|act| act.details(title).state(author).assets(|ass| ass.large_image(artwork))).unwrap();
    }
}


fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));

                    window.eval(r#"
                        __TAURI_INVOKE__("presence", { 
                            playing: document.querySelector("div.playControls__elements").children[1].classList.contains("playing"),
                            title: document.querySelector("div.playbackSoundBadge__titleContextContainer > div > a > :last-child").innerText,
                            author: document.querySelector("div.playbackSoundBadge__titleContextContainer > a").innerText,
                            artwork: document.querySelector("div.playbackSoundBadge > a > div > span").style.backgroundImage.match(/url\("(.*)"\)/)[1]
                        })
                    "#).unwrap();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![presence])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}