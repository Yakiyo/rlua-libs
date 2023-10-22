use rlua::Result as LuaResult;
use rlua::{Context, Table};

/// Takes a lua context, a module name, and a lua table
/// and sets the table with the name as key to the global
/// package.loaded table
pub(crate) fn def_mod<'a>(ctx: Context<'a>, name: &str, module: Table<'a>) -> LuaResult<()> {
    let globals = ctx.globals();
    let package: Table = globals.get("package")?;
    let loaded: Table = package.get("loaded")?;
    loaded.set(name, module)
}
