use std::str::FromStr;

use env_logger::{Builder, Target};
use log::LevelFilter;
use rlua::{Context, Result as LuaResult, UserData};
pub(super) struct LogBuilder(Builder);

impl std::ops::Deref for LogBuilder {
    type Target = Builder;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LogBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl UserData for LogBuilder {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method_mut("parse_env", |_, builder, env: String| {
            builder.parse_env(env);
            Ok(())
        });
        methods.add_method_mut("filter_level", |_, builder, level: String| {
            let level =
                LevelFilter::from_str(level.as_str()).map_err(|err| rlua::Error::external(err))?;
            builder.filter_level(level);
            Ok(())
        });
        methods.add_method_mut("target", |_, builder, target: String| {
            let target = match target.to_ascii_lowercase().as_str() {
                "stdout" => Target::Stdout,
                "stderr" => Target::Stderr,
                _ => {
                    return Err(rlua::Error::external(format!(
                        "{} is not a valid target. Must be one of stdout, stderr",
                        target
                    )))
                }
            };
            builder.target(target);
            Ok(())
        });
        methods.add_method_mut("init", |_, builder, _: ()| {
            builder.init();
            Ok(())
        });
    }
}

/// Create a new log builder
///
/// ```lua
/// local builder = log.builder()
/// ```
pub(super) fn create_builder(_: Context, _: ()) -> LuaResult<LogBuilder> {
    Ok(LogBuilder(Builder::new()))
}
