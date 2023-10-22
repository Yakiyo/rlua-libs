use reqwest::blocking::{Client, RequestBuilder};
use reqwest::Method;
use rlua::{Context, Result as LuaResult, UserData};
use std::time::Duration;

/// A request builder
///
/// This can be used to create a more complex request builder
/// and then passed to `client:do_request(request)`
#[derive(Debug)]
pub(super) struct Request(RequestBuilder);

impl Request {
    pub(super) fn inner(self) -> RequestBuilder {
        self.0
    }
}

impl Clone for Request {
    fn clone(&self) -> Self {
        // FIXME: unwrap here is a potential time bomb. gotta fix this someday
        Request(self.0.try_clone().unwrap())
    }
}

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
    (verb, url, body, timeout): (String, String, Option<String>, Option<u64>),
) -> LuaResult<Request> {
    let method = Method::from_bytes(&verb.as_bytes()).map_err(|e| rlua::Error::external(e))?;
    let rb = Client::new().request(method, url);
    let rb = if let Some(body) = body {
        rb.body(body)
    } else {
        rb
    };
    let rb = if let Some(timeout) = timeout {
        rb.timeout(Duration::new(timeout, 0))
    } else {
        rb
    };
    Ok(Request(rb))
}
