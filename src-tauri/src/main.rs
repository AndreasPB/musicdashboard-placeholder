#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use reqwest::Error;
use serde::Deserialize;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command]
fn hello() -> String {
    "Hello, world!".to_string()
}

#[derive(Deserialize)]
struct Genre {
    label: String,
    value: String,
}

/// let ip = reqwest::get("http://httpbin.org/ip")
///     .await?
///     .json::<Ip>()
///     .await?;

#[tauri::command]
async fn get_genres() -> String {
    let url = "https://api.streamchaser.tv/genres/".to_string();

    // FIXME: Should use the genre struck somehow - Rust is crazy
    match reqwest::get(&url).await {
        Ok(res) => match res.text().await {
            Ok(genres) => genres,
            Err(e) => e.to_string()
        }
        Err(e) => e.to_string()
    }

    // match reqwest::get(&url).await?.json::<Vec<Genre>>().await {
    //     Ok(genres) => Ok(genres),
    //     Err(_) => "lort".to_string()

    // }
    
    // match reqwest::get(&url).await {
    //     Ok(res) => match res.json().await {
    //         Ok(text) => return vec![text],
    //         Err(e) => println!("{}", e),
    //     },
    //     Err(e) => println!("{}", e),
    // }
    // return "lol du tabte".to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, hello, get_genres])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
