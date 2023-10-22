http = require("http")

-- define client
local client = http.client()

-- client get request
local res = client:get("https://httpbin.org/get")
if not (res:status() == 200) then
	error("Did not receive status code 200 on res")
end

-- client post request
local res2 = client:post("https://httpbin.org/post", "hi")
if not (res2:status() == 200) then
	error("Did not receive status code 200 on res2")
end

-- client do_request
local request = http.request("GET", "https://httpbin.org/get")
local res3 = client:do_request(request)
if not (res3:status() == 200) then
	error("Did not receive status code 200 on res3")
end

if not (res:status() == res3:status()) then
	error("This should be equal")
end

-- top level get function
-- equivalent to creating a client and then doing a get request
local get_resp = http.get("https://httpbin.org/get")
if not (get_resp:status() == 200) then
	error("Did not receive status code 200 on get_resp")
end
