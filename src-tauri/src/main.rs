#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};
use std::sync::Mutex;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use lazy_static::lazy_static;
use tauri::Manager;

lazy_static! {
    static ref RPC: Mutex<DiscordIpcClient> = Mutex::new(DiscordIpcClient::new("1156699732765310976").unwrap());
}

#[tauri::command]
fn presence(_playing: bool, title: &str, author: &str, artwork: &str, link: &str) {
    let mut client = RPC.lock().unwrap();
    client.set_activity(activity::Activity::new()
        .state(&title)
        .details(&author)
        .assets(
            activity::Assets::new()
                .large_image(&artwork)
        )
        .buttons(vec![
            activity::Button::new(
                "Song Link",
                &link),
            activity::Button::new(
                "Source Code",
                "https://github.com/dxxxxy/scrp"
            )
        ])
    ).unwrap();
}


fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            //spawn thread for rpc connection
            thread::spawn(move || {
                let mut client = RPC.lock().unwrap();
                client.connect().unwrap();
            });

            //spawn thread for scraping track info
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));

                    window.eval(r#"
                        //scrape track info
                        window.track = {
                            playing: document.querySelector("div.playControls__elements").children[1].classList.contains("playing"),
                            title: document.querySelector("div.playbackSoundBadge__titleContextContainer > div > a > :last-child").innerText,
                            author: document.querySelector("div.playbackSoundBadge__titleContextContainer > a").innerText,
                            artwork: document.querySelector("div.playbackSoundBadge > a > div > span").style.backgroundImage.match(/url\("(.*)"\)/)[1],
                            link: document.querySelector("div.playbackSoundBadge > a").href
                        }

                        //send only if track has changed
                        if (JSON.stringify(window.track) !== JSON.stringify(window.lastTrack)) __TAURI_INVOKE__("presence", structuredClone(window.track))

                        //save last track
                        window.lastTrack = window.track
                    "#).unwrap();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![presence])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}