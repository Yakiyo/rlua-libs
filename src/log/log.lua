local log = require("log")

local builder = log.builder()

builder:parse_env("RUST_LOG")
builder:filter_level("trace")
builder:target("stderr")
builder:init()

log.error("error")
log.warn("warn")
log.info("info")
log.debug("debug")
log.trace("trace")
