use rlua::{Context, Lua, Result as LuaResult};

fn replace_all(_: Context, (s, from, to): (String, String, String)) -> LuaResult<String> {
    Ok(s.replace(&from, &to))
}

fn replace(_: Context, (s, from, to, count): (String, String, String, usize)) -> LuaResult<String> {
    Ok(s.replacen(&from, &to, count))
}

fn split(_: Context, (s, sep): (String, String)) -> LuaResult<Vec<String>> {
    Ok(s.split(&sep).map(String::from).collect())
}

fn to_lower(_: Context, s: String) -> LuaResult<String> {
    Ok(s.to_lowercase())
}

fn to_upper(_: Context, s: String) -> LuaResult<String> {
    Ok(s.to_uppercase())
}

fn trim_space(_: Context, s: String) -> LuaResult<String> {
    Ok(s.trim().into())
}
fn trim_start(_: Context, (s, cutset): (String, String)) -> LuaResult<String> {
    Ok(s.trim_start_matches(&cutset).into())
}
fn trim_end(_: Context, (s, cutset): (String, String)) -> LuaResult<String> {
    Ok(s.trim_end_matches(&cutset).into())
}

fn contains(_: Context, (s, pattern): (String, String)) -> LuaResult<bool> {
    Ok(s.contains(&pattern))
}

pub fn load(lua: &Lua) -> LuaResult<()> {
    lua.context(|ctx| {
        let table = ctx.create_table()?;

        table.set("replace_all", ctx.create_function(replace_all)?)?;
        table.set("replace", ctx.create_function(replace)?)?;
        table.set("split", ctx.create_function(split)?)?;
        table.set("to_lower", ctx.create_function(to_lower)?)?;
        table.set("to_upper", ctx.create_function(to_upper)?)?;
        table.set("trim", ctx.create_function(trim_space)?)?;
        table.set("trim_start", ctx.create_function(trim_start)?)?;
        table.set("trim_end", ctx.create_function(trim_end)?)?;
        table.set("contains", ctx.create_function(contains)?)?;
        crate::util::def_mod(ctx, "strings", table)
    })
}
