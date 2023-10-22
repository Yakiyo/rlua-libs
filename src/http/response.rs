use reqwest::blocking::Response;
use rlua::UserData;

/// An http response body
///
/// This is returned by any of the fetch requests within an
/// http client. It contains two methods, `body()` for getting
/// the response body, and `status()` for getting the status
/// code of the response.
///
/// ```lua
/// local client = http.client()
/// local resp = client.get("http://example.com")
/// print(resp.status()) -- type int
/// print(resp.body())   -- type string
/// ```
#[derive(Debug, Clone)]
pub(super) struct HttpResponse {
    /// Response body
    pub(super) body: String,
    /// Response status code
    pub(super) status: u16,
}

impl From<Response> for HttpResponse {
    fn from(res: Response) -> Self {
        Self {
            status: res.status().as_u16(),
            // TODO: there should be a better way to handle this ig
            body: res.text().unwrap_or("".to_string()),
        }
    }
}

impl UserData for HttpResponse {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("body", |_, res, _: ()| Ok(res.body.clone()));
        methods.add_method("status", |_, res, _: ()| Ok(res.status));
    }
}
