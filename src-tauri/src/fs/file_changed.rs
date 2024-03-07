use std::fs;
use std::io;
use std::thread::JoinHandle;
use std::time::SystemTime;

fn get_modification_time(file_path: &str) -> io::Result<SystemTime> {
    fs::metadata(file_path).and_then(|metadata| metadata.modified())
}

pub fn run_on_file_change<F>(file_path: String, mut runc: F) -> JoinHandle<Result<(), io::Error>>
where
    F: 'static + FnMut() + Send,
{
    std::thread::spawn(move || {
        let mut last_modification_time = get_modification_time(file_path.as_str())?;
        loop {
            if let Ok(modification_time) = get_modification_time(file_path.as_str()) {
                if modification_time != last_modification_time {
                    println!("File has changed!");
                    runc();
                    last_modification_time = modification_time;
                }
            }
        }
    })
}
