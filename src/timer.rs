use crate::config::Config;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write};

// Теперь функция принимает второй аргумент: running
pub fn run_timer(config: Config, running: Arc<AtomicBool>, playing: Arc<AtomicBool>) {
    while running.load(Ordering::SeqCst) {
        println!("Таймер запущен на {} секунд.", config.duration_secs);
        visualize_timer(config.duration_secs, running.clone());

        if !running.load(Ordering::SeqCst) {
            break; // Прерывание, если флаг running изменен на false
        }

        playing.store(true, Ordering::SeqCst);

        let playing_clone = Arc::clone(&playing);
        let running_clone = running.clone();
        let handle = thread::spawn(move || {
            while playing_clone.load(Ordering::SeqCst) && running_clone.load(Ordering::SeqCst) {
                if let Err(e) = crate::sound::play_sound(playing_clone.clone()) {
                    eprintln!("Ошибка при воспроизведении звука: {}", e);
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("Нажмите Enter, чтобы остановить воспроизведение...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка при чтении строки");
        playing.store(false, Ordering::SeqCst);

        handle.join().unwrap();
        println!("Воспроизведение остановлено. Таймер перезапущен.");
    }
}

fn visualize_timer(duration: u64, running: Arc<AtomicBool>) {
    let start = Instant::now();
    let duration = Duration::from_secs(duration);

    while Instant::now() - start < duration {
        if !running.load(Ordering::SeqCst) {
            break; // Прерывание, если флаг running изменен на false
        }
        let elapsed = Instant::now() - start;
        let remaining = duration - elapsed;
        print!("\rОсталось: {:02} сек.", remaining.as_secs());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!(); // Для перехода на новую строку после окончания таймера
}