use gfx;

pub struct Engine {
    gfx: gfx::Instance,
}

impl Engine {
    fn new() -> Engine {
        Engine { gfx: gfx::Instance::new() }
    }

    fn init(&mut self) -> Result<(), String> {
        try!(self.gfx.init());
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), String> {
        try!(self.gfx.close());
        Ok(())
    }

    pub fn do_frame(&mut self) {
        
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.close().expect("Error shutting down engine");
    }
}

// public interface
pub fn init() -> Result<Engine, String> {
    let mut eng = Engine::new();
    try!(eng.init());
    Ok(eng)
}
