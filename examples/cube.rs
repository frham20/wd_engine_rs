extern crate wd;
//use std::default;

#[allow(unused_variables)]
fn main() {
    println!("WD Engine v0.1.0");
    
    let mut engine = wd::init()
        .expect("error intializing engine");

    let mut timer = wd::Timer::new();
    timer.start();
    let mut i = 0;
    for x in 1..100_000_000 { i += 1; }
    timer.stop();
    println!("Operation took {} after {} iterations", timer, i);

    engine.close()
        .expect("error closing engine");
}
