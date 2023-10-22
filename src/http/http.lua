http = require("http")

-- define client
local client = http.client()

-- client get request
local res = client:get("https://httpbin.org/get")
print("res 1")
print(res:body())

-- client post request
local res2 = client:post("https://httpbin.org/post", "hi")
print("res 2")
print(res2:body())

-- client do_request
local request = http.request("GET", "https://httpbin.org/get")
local res3 = client:do_request(request)
print("res 3")
print(res3:body())

print("status equality")
print(res:status() == res3:status())

-- top level get function 
-- equivalent to creating a client and then doing a get request
local get_resp = http.get("https://httpbin.org/get")
print("get resp")
print(get_resp:body())
