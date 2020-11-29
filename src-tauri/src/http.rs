use std::io::Read;
use serde::{Deserialize, Serialize};
use std::io;

/// Key value pairs serialized as length 2 json string arrays ['key', 'value']
type KeyVal = Vec<String>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequest {
    pub request_id: String,
    /// The request method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, CONNECT or TRACE)
    pub method: String,
    /// The request URL
    pub url: String,
    /// The request query params
    pub params: Vec<KeyVal>,
    /// The request headers
    pub headers: Vec<KeyVal>,
    /// The request body
    pub body: Option<Vec<u8>>,

    // TODO: Implement the rest of this.
    /// Whether to follow redirects or not
    pub follow_redirects: Option<bool>,
    /// Max number of redirections to follow
    pub max_redirections: Option<u32>,
    /// Connect timeout for the request
    pub connect_timeout: Option<u64>,
    /// Read timeout for the request
    pub read_timeout: Option<u64>,
    /// Timeout for the whole request
    pub timeout: Option<u64>,
    /// Whether the request will announce that it accepts compression
    pub allow_compression: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse {
    pub request_id: String,
    #[serde(with = "base64")]
    pub body: Vec<u8>,
    /// Headers are returned as an array of arrays, with [['key', 'value'], ...]
    pub headers: Vec<Vec<String>>,
    pub url: String,
    pub status_code: u16,
    pub status_line: String,
}

pub fn do_request(json_req: HttpRequest) -> io::Result<HttpResponse> {
    let mut request = ureq::request(&json_req.method, &json_req.url);

    for header in json_req.headers.iter() {
        if header.len() == 2 {
            request.set(header[0].as_ref(), header[1].as_ref());
        }
    }

    for param in json_req.params.iter() {
        if param.len() == 2 {
            request.query(param[0].as_ref(), param[1].as_ref());
        }
    }

    let res = request.call();

    // Handle response

    let mut headers: Vec<KeyVal> = Vec::new();
    for header in res.headers_names() {
        for val in res.all(header.as_ref()).iter() {
            headers.push(vec![header.clone(), val.to_owned().to_string()]);
        }
    }
    let status_code =  res.status();
    let status_line = res.status_line().to_owned();
    let url = res.get_url().to_owned();

    let mut reader = res.into_reader();
    let mut body: Vec<u8> = vec![];
    reader.read_to_end(&mut body)?;

    Ok(HttpResponse {
        request_id: json_req.request_id,
        status_code,
        status_line,
        url,
        headers,
        body,
    })
}

mod base64 {
    extern crate base64;
    use serde::Serializer;

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&base64::encode(bytes))

        // Could also use a wrapper type with a Display implementation to avoid
        // allocating the String.
        //
        // serializer.collect_str(&Base64(bytes))
    }

    // pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    //     where D: Deserializer<'de>
    // {
    //     let s = <&str>::deserialize(deserializer)?;
    //     base64::decode(s).map_err(de::Error::custom)
    // }
}

