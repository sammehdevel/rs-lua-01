use std::fs;
use mlua::Lua;

pub fn run_lua(name: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing {}", name);
    let lua = Lua::new();
    let lua_file = filename;
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