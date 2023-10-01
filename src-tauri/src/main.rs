#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use tauri::Manager;
use lazy_static::lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref RPC: Mutex<DiscordIpcClient> = Mutex::new(DiscordIpcClient::new("1156699732765310976").unwrap());
}

#[tauri::command]
fn presence(playing: bool, title: &str, author: &str, artwork: &str) {
    print!("Playing: {}, Title: {}, Author: {}, Artwork: {}", playing, title, author, artwork);

    let mut client = RPC.lock().unwrap();
    client.set_activity(activity::Activity::new()
        .state(&title)
        .details(&author)
        .assets(
            activity::Assets::new()
                .large_image(&artwork)
                // .large_text("Large text"),
        )
        // .buttons(vec![activity::Button::new(
        //     "A button",
        //     "https://github.com",
        // )])
    ).unwrap();
}


fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            thread::spawn(move || {
                let mut client = RPC.lock().unwrap();
                client.connect().unwrap();
            });

            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));

                    //get state from window
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