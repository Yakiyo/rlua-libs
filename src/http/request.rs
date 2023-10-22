use std::time::Duration;

use reqwest::blocking::{Client, RequestBuilder};
use reqwest::Method;
use rlua::{Context, Result as LuaResult, UserData};

/// A request builder
///
/// This can be used to create a more complex request builder
/// and then passed to `client:do_request(request)`
pub(super) struct Request(RequestBuilder);

impl std::ops::Deref for Request {
    type Target = RequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UserData for Request {}

/// Lua function to create a request builder
///
/// This takes the http verb [GET, POST, PATCH, ...],
/// the url to make the request to, and an optional body.
/// ```lua
/// http.request("GET", "https://example.com")
/// http.request("POST", "https://some-site.com", "here goes the body")
/// ```
pub(super) fn create_request(
    _: Context,
    (verb, url, body): (String, String, Option<String>),
) -> LuaResult<Request> {
    let method = Method::from_bytes(&verb.as_bytes()).map_err(|e| rlua::Error::external(e))?;
    let rb = Client::new().request(method, url);
    let rb = if let Some(body) = body {
        rb.body(body)
    } else {
        rb
    };
    Ok(Request(rb))
}
