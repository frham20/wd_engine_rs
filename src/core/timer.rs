mod inner {
    use kernel32;

    pub const US_TO_MS : f64 = 1.0f64 / 1_000f64;
    pub const US_TO_S : f64  = 1.0f64 / 1_000_000f64;

    pub static mut ticks_frequency : u64 = 0u64;
    
    #[inline]
    pub fn query_performance_frequency() -> u64 {
        unsafe {
            let mut frequency = 0;
            kernel32::QueryPerformanceFrequency(&mut frequency);
            frequency as u64
        }      
    }

    #[inline]
    pub fn query_performance_counter() -> u64 {
        unsafe {
            let mut ticks = 0;
            kernel32::QueryPerformanceCounter(&mut ticks);
            ticks as u64
        }      
    } 
}

#[derive(Default)]
pub struct Timer {
    ticks : u64,
}

impl Timer {
    pub fn init() {
        unsafe {
            inner::ticks_frequency = inner::query_performance_frequency();
        }
    }
    
    #[inline]
    pub fn start(&mut self) {
        self.ticks = inner::query_performance_counter();
    }

    #[inline]
    pub fn stop(&mut self) -> u64 {
        let stop_ticks = inner::query_performance_counter();
        self.ticks = stop_ticks - self.ticks;

        //transform to microseconds (us)
        self.ticks *= 1_000_000;
        unsafe {
            self.ticks /= inner::ticks_frequency;
        }

        self.ticks
    }

    #[inline]
    pub fn get_s(&mut self) -> f64 {
        (self.ticks as f64) * inner::US_TO_S
    }

    #[inline]
    pub fn get_ms(&mut self) -> f64 {
        (self.ticks as f64) * inner::US_TO_MS
    }

    #[inline]
    pub fn get_us(&mut self) -> u64 {
        self.ticks
    }    
}

/*
impl Display for Timer {

}*/