use mlua::Lua;
use std::fs;
use rs_lua_01::run_lua;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_lua("lua-04-tables", "lua/tables.lua")
}
