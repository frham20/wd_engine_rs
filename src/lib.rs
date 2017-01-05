//crates
extern crate glfw;
extern crate log;

//standard library imports

// wd library imports
mod core;
mod gfx;

pub use core::Engine;


pub fn init() -> Result<Engine, String> {
    let mut eng = Engine::new();
    try!(eng.init());
    Ok(eng)
}

