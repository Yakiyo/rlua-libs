use reqwest::blocking::Client as RClient;
use rlua::UserData;
use std::sync::Arc;

use super::response::HttpResponse;

pub(super) struct HttpClient(RClient);

impl std::ops::Deref for HttpClient {
    type Target = RClient;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UserData for HttpClient {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("get", |_, client, url: String| {
            client
                .get(&url)
                .send()
                .map_err(|e| rlua::Error::ExternalError(Arc::new(e)))
                .map(|f| Into::<HttpResponse>::into(f))
        });

        methods.add_method(
            "post",
            |_, client, (url, body): (String, Option<String>)| {
                let req = client.post(&url);
                let req = if let Some(body) = body {
                    req.body(body)
                } else {
                    req
                };

                Ok(req
                    .send()
                    .map_err(|e| rlua::Error::ExternalError(Arc::new(e)))
                    .map(|f| Into::<HttpResponse>::into(f)))
            },
        )
    }
}
