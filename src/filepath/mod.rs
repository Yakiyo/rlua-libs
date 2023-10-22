#![doc = include_str!("README.md")]

use rlua::{Context, Lua, Result as LuaResult};
use std::path::Path;

/// get extension of a file path
fn ext(_: Context, path: String) -> LuaResult<Option<String>> {
    Ok(Path::new(&path)
        .extension()
        .map(|f| f.to_str())
        .flatten()
        .map(String::from))
}

/// get basename of a file path
fn basename(_: Context, path: String) -> LuaResult<Option<String>> {
    Ok(Path::new(&path)
        .file_name()
        .map(|f| f.to_str())
        .flatten()
        .map(String::from))
}

/// get dir name of a file path
fn dir(_: Context, path: String) -> LuaResult<Option<String>> {
    Ok(Path::new(&path)
        .parent()
        .map(Path::to_str)
        .flatten()
        .map(String::from))
}

/// join multiple paths
fn join(_: Context, paths: Vec<String>) -> LuaResult<Option<String>> {
    if paths.len() < 1 {
        return Ok(None);
    }
    let mut paths = paths.iter().map(Path::new);
    let mut pb = std::path::PathBuf::new();
    while let Some(path) = paths.next() {
        pb = pb.join(path);
    }
    Ok(pb.to_str().map(String::from))
}

pub fn load(lua: &Lua) -> LuaResult<()> {
    lua.context::<_, LuaResult<()>>(|ctx| {
        let fp = ctx.create_table()?;
        fp.set("ext", ctx.create_function(ext)?)?;
        fp.set("basename", ctx.create_function(basename)?)?;
        fp.set("dir", ctx.create_function(dir)?)?;
        fp.set("join", ctx.create_function(join)?)?;

        crate::util::def_mod(ctx, "filepath", fp)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let lua = Lua::new();
        load(&lua).unwrap();
        lua.context(|ctx| {
            ctx.load(include_str!("filepath.lua")).exec().unwrap();
        });
    }
}
