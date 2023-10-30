use rlua::{Lua, Result as LuaResult};

mod builder;

pub fn load(lua: &Lua) -> LuaResult<()> {
    lua.context(|ctx| {
        let table = ctx.create_table()?;
        table.set("builder", ctx.create_function(builder::create_builder)?)?;

        table.set(
            "error",
            ctx.create_function(|_, msg: String| Ok(log::error!("{msg}")))?,
        )?;

        table.set(
            "warn",
            ctx.create_function(|_, msg: String| Ok(log::warn!("{msg}")))?,
        )?;

        table.set(
            "info",
            ctx.create_function(|_, msg: String| Ok(log::info!("{msg}")))?,
        )?;

        table.set(
            "debug",
            ctx.create_function(|_, msg: String| Ok(log::debug!("{msg}")))?,
        )?;

        table.set(
            "trace",
            ctx.create_function(|_, msg: String| Ok(log::trace!("{msg}")))?,
        )?;

        crate::util::def_mod(ctx, "log", table)
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
            ctx.load(include_str!("log.lua")).exec().unwrap();
        });
    }
}
