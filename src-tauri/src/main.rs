use tauri::{
    api::process::{Command, CommandEvent},
    Manager,
};

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JS!");
    // tauri::async_runtime::spawn(async move {
    //     let (mut rx, mut child) = Command::new_sidecar("multiwormtracker_app")
    //         .expect("failed to setup `app` sidecar")
    //         .spawn()
    //         .expect("Failed to spawn packaged node");
    // let mut i = 0;
    // while let Some(event) = rx.recv().await {
    //     if let CommandEvent::Stdout(line) = event {
    //         window
    //             .emit("message", Some(format!("'{}'", line)))
    //             .expect("failed to emit event");
    //         i += 1;
    //         if i == 4 {
    //             child.write("message from Rust\n".as_bytes()).unwrap();
    //             i = 0;
    //         }
    //     }
    // }
    // });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
