#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod cmd;
mod http;
mod sqlite;

use std::sync::{Arc, Mutex};
use crate::sqlite::DB;

fn main() {

    let db = Arc::new(Mutex::new(DB::new()));

    tauri::AppBuilder::new()
        .invoke_handler(move |_webview, arg| {
            use cmd::Cmd::*;

            let db = db.clone();

            match serde_json::from_str(arg) {
                Err(e) => {
                    Err(e.to_string())
                }
                Ok(command) => {
                    match command {
                        ReposeHttpRequest { request, callback, error } => {
                            tauri::execute_promise(_webview, move || {
                                    http::do_request(request)
                                        .map_err(|e| { anyhow::Error::from(e) })
                                }, callback, error)
                        }
                        SqliteCommand { command, callback, error } => {
                            tauri::execute_promise(_webview, move || {
                                // TODO: Could be better to run this in it's own thread instead of
                                // locking in the worker pool threads.
                                let db = db.lock().unwrap();
                                db.handle_command(command)
                                    .map_err(|e| { anyhow::Error::from(e) })
                            }, callback, error)
                        }
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}
