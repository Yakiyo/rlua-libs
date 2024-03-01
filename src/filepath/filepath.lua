--[[
-- This test file is taken from https://github.com/metafates/mangal-lua-libs/tree/main/filepath
--]]

local filepath = require("filepath")

-- filepath.ext() : file extension
local result = filepath.ext("/var/tmp/file.name")
if not (result == "name") then
	error("ext")
end

-- filepath.basename() : filename of the file indicated by the path
local result = filepath.basename("/var/tmp/file.name")
if not (result == "file.name") then
	error("basename")
end

-- filepath.dir() : directory name of the file
local result = filepath.dir("/var/tmp/file.name")
if not (result == "/var/tmp") then
	error("dir")
end

-- filepath.join() : concatonate multiple file paths
local result = filepath.join({ "var", "tmp", "file.name" })
if not ((result == "var/tmp/file.name") or (result == "var\\tmp\\file.name")) then
	error(result)
end

-- filepath.path_sep : filepath separator -> / for unix, \ for windows
local result = filepath.path_sep
if not ((result == "/") or (result == "\\")) then
	error(result)
end