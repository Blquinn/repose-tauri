use serde::Deserialize;

use crate::http;
use crate::sqlite;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  ReposeHttpRequest {
    request: http::HttpRequest,
    callback: String,
    error: String,
  },
  SqliteCommand {
    command: sqlite::Commands,
    callback: String,
    error: String,
  },
}
