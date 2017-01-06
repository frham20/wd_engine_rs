//crates
extern crate glfw;
extern crate log;
extern crate kernel32;

//standard library imports

// wd library imports
mod core;
mod gfx;

pub use core::Engine;
pub use core::Timer;

pub fn init() -> Result<Engine, String> {
    let mut eng = Engine::new();
    try!(eng.init());
    Ok(eng)
}

