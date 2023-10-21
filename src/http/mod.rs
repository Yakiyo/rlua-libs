use reqwest::blocking::Client;
use rlua::{Context, Lua, Result as LuaResult, UserData, UserDataMethods};
use std::{ops::Deref, sync::Arc};

/// An http client
pub struct HttpClient(Client);

/// Http response
pub struct HttpResponse {
    pub body: String,
    pub code: u16,
    // pub headers:
}

impl From<reqwest::blocking::Response> for HttpResponse {
    fn from(value: reqwest::blocking::Response) -> Self {
        HttpResponse {
            // TODO: should not unwrap here tbh, but oh well. Should fix this in the future
            code: value.status().as_u16(),
            body: value.text().unwrap(),
        }
    }
}

impl UserData for HttpResponse {}

impl Deref for HttpClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn client_get(_: rlua::Context<'_>, client: &HttpClient, url: String) -> LuaResult<HttpResponse> {
    let response: HttpResponse = client
        .get(&url)
        .send()
        .map_err(|e| rlua::Error::ExternalError(Arc::new(e)))?
        .into();
    Ok(response)
}

/// Create a new client instantiation
pub fn create_client(_: Context, _: ()) -> LuaResult<HttpClient> {
    Ok(HttpClient(Client::new()))
}

impl UserData for HttpClient {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("get", client_get);

        methods.add_method("post", |_, client, url: String| {
            let response: HttpResponse = client
                .post(&url)
                .send()
                .map_err(|e| rlua::Error::ExternalError(Arc::new(e)))?
                .into();
            Ok(response)
        });
    }
}

/// Load the http module within the lua context
pub fn load(lua: &Lua) -> LuaResult<()> {
    lua.context::<_, LuaResult<()>>(|ctx| {
        let http_module = ctx.create_table()?;
        // Register the `http.client()` function
        http_module.set("client", ctx.create_function(create_client)?)?;
        // Register the `http.get()` function. This internally creates a new client
        // and invokes its `get` method
        http_module.set(
            "get",
            ctx.create_function(|_, url: String| {
                let client = HttpClient(Client::new());
                let response: HttpResponse = client
                    .get(&url)
                    .send()
                    .map_err(|e| rlua::Error::ExternalError(Arc::new(e)))?
                    .into();
                Ok(response)
            })?,
        )?;
        let globals = ctx.globals();
        globals.set("http", http_module)?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use rlua::*;
    #[test]
    fn lua_http() {
        let lua = Lua::new();
        super::load(&lua).unwrap();
        lua.context(|ctx| {
            ctx.load(
                r#"
            local http = require("http")
            local response_get = http.get("https://example.com")

            print(response_get)
            "#,
            )
            .exec()
            .unwrap();
        })
    }
}
