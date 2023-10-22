# http

## Functions
- `client()` - returns a reusable http client
```lua
local client = http.client()
```

- `request(method, url, [body, timeout])` - creates a request, that can be executed through `client:do_request`. body and timeout (in seconds) are optional. `method` must be one of [`request::Method`](https://docs.rs/reqwest/latest/reqwest/struct.Method.html)
```lua
local request = http.request("POST", "https://httpbin.com/post", "some body", 120)
```

- `get(url)` - creates a new `client` and makes a get request
```lua
local res = http.get("https://httpbin.com/get")
```

- `post(url, [body])` - creates a new `client` and makes a post request
```lua
local res = http.post("https://httpbin.com/post", "body")
```

> **NOTE:** For multiple requests, it is recommended to create a client instance and reuse it, instead of doing `get` and `post` multiple times

## Methods
### client
- `get(url)` - similar to `http.get`
```lua
local client = http.client()
local res = client:get("url")
```

- `post(url, [body])` - similar to `http.post`
```lua
local client = http.client()
local res = client:post("url", "optional body")
```

- `do_request(method, url, [body, timeout])` - executes a `request`. timeout is in seconds
```lua
local client = http.client()
local request = http.request("POST", "url", "body", 120)
local res = client:do_request(request)
```

### response
- `body()` - response body
- `status()` - response status code
```lua
local response = http.get("url")
print(response.body(), response.status())
```