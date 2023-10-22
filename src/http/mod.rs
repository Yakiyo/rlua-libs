#![doc = include_str!("README.md")]

mod client;
mod request;
mod response;

use self::{client::HttpClient, response::HttpResponse};
use reqwest::blocking::Client as RClient;
use rlua::{Context, Lua, Result as LuaResult};

/// Load the http module in the lua instance.
///
/// The following variables become available
/// ```lua
/// http.client()
/// http.request("VERB", "url", "body", timeout) -- timeout is a u64 int
/// http.get("url")
/// http.post("url", "body")
/// ```
pub fn load(lua: Lua) -> LuaResult<()> {
    lua.context::<_, LuaResult<()>>(|ctx| {
        let http = ctx.create_table()?;
        // register `http.client()`
        http.set("client", ctx.create_function(client::create_client)?)?;
        // register `http.request()`
        http.set("request", ctx.create_function(request::create_request)?)?;
        // register top level get request
        http.set("get", ctx.create_function(get)?)?;
        // register top level post request
        http.set("post", ctx.create_function(post)?)?;

        ctx.globals().set("http", http)
    })
}

/// The `http.get` function
///
/// This internally creates a client and makes the GET request.
/// For doing multiple requests, it is recommended to generate
/// a client and then use it. This is equivalent to doing
/// ```lua
/// local client = http.client()
/// client.get("url")
/// ```
fn get(_: Context, url: String) -> LuaResult<HttpResponse> {
    let client: HttpClient = RClient::new().into();
    client.get_func(url)
}

/// The `http.post` function
///
/// This internally creates a client and makes the post request.
/// For doing multiple requests, it is recommended to generate
/// a client and then use itThis is equivalent to doing
/// ```lua
/// local client = http.client()
/// client.post("url", "body")
/// ```
fn post(_: Context, (url, body): (String, Option<String>)) -> LuaResult<HttpResponse> {
    let client: HttpClient = RClient::new().into();
    client.post_func(url, body)
}
