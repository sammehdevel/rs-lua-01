use mlua::Lua;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing lua-01-basic");
    let lua = Lua::new();
    let lua_file = "lua/hello.lua";
    println!("Loading {lua_file}");
    let chunk = fs::read_to_string(lua_file)?;
    println!("Running {lua_file}");
    let header = format!(
        "{} {} {}",
        "=".repeat((32 - lua_file.len()) / 2),
        lua_file,
        "=".repeat((32 - lua_file.len()) / 2)
    );
    println!(
        "{}\n\n", header
    );
    lua.load(chunk).exec()?;
    println!("\n\n{}", "=".repeat(header.len()));
    println!("Finished running {lua_file}");
    println!("Exiting...");
    Ok(())
}
