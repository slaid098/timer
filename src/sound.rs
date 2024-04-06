use rand::Rng;
use rodio::OutputStream;
use rodio::{Decoder, Sink};
use std::env;
use std::io;
use std::{fs::File, io::BufReader, path::PathBuf};

fn play_random_sound_from_directory(
    stream_handle: &rodio::OutputStreamHandle,
    directory_path: PathBuf,
) -> io::Result<()> {
    let paths = std::fs::read_dir(directory_path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .map_or(false, |ext| ext == "wav" || ext == "mp3")
        })
        .collect::<Vec<_>>();

    if paths.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No sound files found",
        ));
    }

    let mut rng = rand::thread_rng();
    let sound_path = paths[rng.gen_range(0..paths.len())].clone();
    let file = File::open(sound_path)?;
    let source =
        Decoder::new(BufReader::new(file)).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let sink = Sink::try_new(stream_handle).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

pub fn play_sound() -> io::Result<()> {
    let (_stream, stream_handle) =
        OutputStream::try_default().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Предполагаем, что директория 'sounds' находится в текущей рабочей директории
    let current_dir = env::current_dir()?;
    let sounds_dir = current_dir.join("sounds");

    play_random_sound_from_directory(&stream_handle, sounds_dir)
}
