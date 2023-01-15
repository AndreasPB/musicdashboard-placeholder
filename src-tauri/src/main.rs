#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command]
fn hello() -> String {
    "Hello, world!".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
struct Genre {
    label: String,
    value: String,
}

#[tauri::command]
async fn get_memes() -> Result<Vec<Genre>, String> {
    let url = "https://api.streamchaser.tv/genres/".to_string();

    match reqwest::get(&url).await {
        Ok(res) => match res.json::<Vec<Genre>>().await {
            Ok(genres) => Ok(genres),
            Err(e) => Err(e.to_string()),
        },

        Err(e) => Err(e.to_string()),
    }
}

/// let ip = reqwest::get("http://httpbin.org/ip")
///     .await?
///     .json::<Ip>()
///     .await?;

#[tauri::command]
async fn get_genres() -> Vec<Genre> {
    let url = "https://api.streamchaser.tv/genres/".to_string();

    reqwest::get(&url).await.unwrap().json::<Vec<Genre>>().await.unwrap()

    // FIXME: Should use the genre struck somehow - Rust is crazy
    // match reqwest::get(&url).await {
    //     Ok(res) => match res.json::<Vec<Genre>>().await {
    //         Ok(genres) => Ok(genres),
    //         Err(e) => Err(e.into())
    //     },
    //     Err(e) => Err(e.into())
    // }

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
        .invoke_handler(tauri::generate_handler![greet, hello, get_memes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
