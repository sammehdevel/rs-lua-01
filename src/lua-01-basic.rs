use rs_lua_01::run_lua;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_lua("lua-01-basic", "lua/hello.lua")
}
