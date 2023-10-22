use super::request::Request;
use super::response::HttpResponse;
use reqwest::blocking::Client as RClient;
use rlua::{Context, Result as LuaResult, UserData};
use std::sync::Arc;

pub(super) struct HttpClient(RClient);

impl From<RClient> for HttpClient {
    fn from(value: RClient) -> Self {
        HttpClient(value)
    }
}

impl std::ops::Deref for HttpClient {
    type Target = RClient;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HttpClient {
    pub(super) fn get_func(&self, url: String) -> LuaResult<HttpResponse> {
        self.get(&url)
            .send()
            .map_err(|e| rlua::Error::ExternalError(Arc::new(e)))
            .map(|f| Into::<HttpResponse>::into(f))
    }

    pub(super) fn post_func(&self, url: String, body: Option<String>) -> LuaResult<HttpResponse> {
        let req = self.post(&url);
        let req = if let Some(body) = body {
            req.body(body)
        } else {
            req
        };

        req.send()
            .map_err(|e| rlua::Error::ExternalError(Arc::new(e)))
            .map(|f| Into::<HttpResponse>::into(f))
    }
}

impl UserData for HttpClient {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("get", |_, client, url: String| client.get_func(url));

        methods.add_method(
            "post",
            |_, client, (url, body): (String, Option<String>)| client.post_func(url, body),
        );

        methods.add_method("do_request", |_, client, request: Request| {
            client
                .execute(
                    request
                        .inner()
                        .build()
                        .map_err(|e| rlua::Error::external(e))?,
                )
                .map_err(|e| rlua::Error::external(e))
                .map(|v| Into::<HttpResponse>::into(v))
        });
    }
}

/// Create a new client instantiation
///
/// ```lua
/// local client = http.client()
/// client.get("https://example.com")
/// ```
pub(super) fn create_client(_: Context, _: ()) -> LuaResult<HttpClient> {
    Ok(HttpClient(RClient::new()))
}
