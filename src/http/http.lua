local response_get = http.get("https://example.com")
local client = http.client()
print(response_get:status())

print(client:get("http://example.com"):body())
