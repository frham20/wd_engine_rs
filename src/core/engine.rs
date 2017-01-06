use gfx;
use core::Timer;

pub struct Engine {
    gfx: gfx::Instance,
}

impl Engine {
    pub fn new() -> Engine {
        Engine { gfx: gfx::Instance::new() }
    }

    pub fn init(&mut self) -> Result<(), String> {

        Timer::init();

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
