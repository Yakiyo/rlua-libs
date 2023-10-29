use rlua::{Lua, Result as LuaResult};

mod builder;

pub fn load(lua: &Lua) -> LuaResult<()> {
    lua.context(|ctx| {
        let table = ctx.create_table()?;
        table.set("builder", ctx.create_function(builder::create_builder)?)?;
        Ok(())
    })
}
