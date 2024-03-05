use std::fs::File;
use std::io::{self, BufReader, Error, ErrorKind};
use std::path::Path;
use std::sync::mpsc::{self, Sender};
use std::thread;

use rodio::{Decoder, OutputStream, Sink};

pub enum MusicHandlerCommand {
    Play(String),
    Stop,
    Pause,
    Resume,
}

pub struct MusicHandlerWrapper {
    sender: Sender<MusicHandlerCommand>,
}

impl MusicHandlerWrapper {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut music_handler = MusicHandler::new();
            for command in receiver {
                match command {
                    MusicHandlerCommand::Play(path) => { let _ = music_handler.play(&path); },
                    MusicHandlerCommand::Stop => music_handler.stop(),
                    MusicHandlerCommand::Pause => music_handler.pause(),
                    MusicHandlerCommand::Resume => music_handler.resume(),
                }
            }
        });

        MusicHandlerWrapper { sender }
    }

    pub fn play(&self, path: String) {
        let _ = self.sender.send(MusicHandlerCommand::Play(path));
    }

    pub fn stop(&self) {
        let _ = self.sender.send(MusicHandlerCommand::Stop);
    }

    pub fn pause(&self) {
        let _ = self.sender.send(MusicHandlerCommand::Pause);
    }

    pub fn resume(&self) {
        let _ = self.sender.send(MusicHandlerCommand::Resume);
    }
}

pub struct MusicHandler {
    sink: Option<Sink>,
    stream: Option<OutputStream>,
}

impl MusicHandler {
    pub fn new() -> Self {
        MusicHandler {
            sink: None,
            stream: None,
        }
    }

    pub fn play(&mut self, file_path: &str) -> io::Result<()> {
        let (stream, stream_handle) = OutputStream::try_default().map_err(|e| Error::new(ErrorKind::Other, e))?;
        let file = File::open(Path::new(file_path))?;
        let source = Decoder::new(BufReader::new(file)).map_err(|e| Error::new(ErrorKind::Other, e))?;

        let sink = Sink::try_new(&stream_handle).map_err(|e| Error::new(ErrorKind::Other, e))?;
        sink.append(source);
        self.stream = Some(stream);
        self.sink = Some(sink);

        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
            self.sink = None;
            self.stream = None;
        }
    }

    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }
}
