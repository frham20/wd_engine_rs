//crates
extern crate glfw;
extern crate log;

#[cfg(target_os = "windows")]
extern crate kernel32;
#[cfg(target_os = "windows")]
extern crate user32;

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

