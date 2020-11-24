#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod cmd;
mod http;

use anyhow::Error;

fn main() {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => {
                    Err(e.to_string())
                }
                Ok(command) => {
                    match command {
                        ReposeHttpRequest { request, callback, error } => {
                            tauri::execute_promise(
                                _webview,
                                move || {
                                    println!("Got http request.");
                                    let response = http::do_request(request);
                                    Ok(response)
                                },
                                callback,
                                error,
                            )
                        }
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}
