use rand::Rng;
use rodio::{Decoder, Sink, OutputStream};
use std::env;
use std::io::{self, BufReader};
use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

pub fn play_sound(playing: Arc<AtomicBool>) -> io::Result<()> {
    // Исправление: явно используем `OutputStream` для получения дефолтного аудиопотока
    let (_stream, stream_handle) = OutputStream::try_default().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let current_dir = env::current_dir()?;
    let sounds_dir = current_dir.join("sounds");

    play_random_sound_from_directory(&stream_handle, sounds_dir, playing)
}

fn play_random_sound_from_directory(
    stream_handle: &rodio::OutputStreamHandle,
    directory_path: PathBuf,
    playing: Arc<AtomicBool>,
) -> io::Result<()> {
    let paths = std::fs::read_dir(directory_path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "wav" || ext == "mp3"))
        .collect::<Vec<_>>();

    if paths.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No sound files found"));
    }

    let mut rng = rand::thread_rng();
    let sound_path = paths[rng.gen_range(0..paths.len())].clone();
    let file = File::open(sound_path)?;
    let source = Decoder::new(BufReader::new(file)).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let sink = Sink::try_new(stream_handle).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    sink.append(source);

    while !sink.empty() && playing.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    if !playing.load(Ordering::SeqCst) {
        sink.stop();
    }

    Ok(())
}