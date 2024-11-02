// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::thread::sleep;
use std::time::Duration;

#[tauri::command]
async fn custom_command(message: &str) -> Result<String, ()> {
    println!("From JS: {message}");

    for _ in 0..30 {
        sleep(Duration::from_secs(1));
    }

    //Awokawokawok bisa njir

    Ok(format!("{message}"))
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
enum ProgressEvent {
    Progress { chunk: i32 },
}

#[tauri::command]
async fn progress_channel(channel: tauri::ipc::Channel<ProgressEvent>) {
    for i in 1..=6 {
        channel.send(ProgressEvent::Progress { chunk: 16 }).unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(
            (i * 100 + 200).try_into().unwrap(),
        ))
        .await;
    }
    channel.send(ProgressEvent::Progress { chunk: 4 }).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![custom_command, progress_channel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
