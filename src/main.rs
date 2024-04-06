use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod config;
mod sound;
mod timer;

fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let playing = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let p = playing.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        p.store(false, Ordering::SeqCst);
        println!("\nЗавершение программы...");
    })?;

    let config = config::Config::new().expect("Failed to load or create config");

    while running.load(Ordering::SeqCst) {
        timer::run_timer(config.clone(), running.clone(), playing.clone());
    }
    println!("Программа завершена.");
    Ok(())
}
