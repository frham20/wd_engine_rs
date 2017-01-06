extern crate wd;
//use std::default;

#[allow(unused_variables)]
fn main() {
    println!("WD Engine v0.1.0");
    
    let mut engine = wd::init()
        .expect("error intializing engine");

    let mut timer : wd::Timer = Default::default();
    timer.start();
    let mut i = 0;
    for x in 1..1_000_000_000 { i += 1; }
    timer.stop();
    println!("Operation took:\n   {:?} us\n   {:?} ms\n   {:?} s\n   after {:?} iterations", timer.get_us(), timer.get_ms(), timer.get_s(), i);

    engine.close()
        .expect("error closing engine");
}
