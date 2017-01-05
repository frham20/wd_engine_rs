extern crate wd;

fn main() {
    println!("WD Engine v0.1.0");
    
    let mut engine = wd::init()
        .expect("error intializing engine");   

    engine.close()
        .expect("error closing engine");
}
