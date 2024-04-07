// use rodio::{source::Source, OutputStream};
// use std::fs::File;
// use std::time::Duration;
// use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
// use symphonia::core::formats::{FormatOptions, FormatReader};
// use symphonia::core::io::MediaSourceStream;
// use symphonia::core::meta::MetadataOptions;
// use symphonia::core::probe::Hint;
// use symphonia::default::{get_codecs, get_probe};

// pub struct MusicHandler {
//     format_reader: Option<Box<dyn FormatReader>>,
//     decoder: Option<Box<dyn symphonia::core::codecs::Decoder>>,
//     output_stream: OutputStream,
//     output_stream_handle: rodio::OutputStreamHandle,
// }

// impl MusicHandler {
//     pub fn new() -> Self {
//         let (output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
//         MusicHandler {
//             format_reader: None,
//             decoder: None,
//             output_stream,
//             output_stream_handle,
//         }
//     }

//     pub fn play(&mut self, file_path: &str) -> Result<(), String> {
//         let file = File::open(file_path).map_err(|e| e.to_string())?;
//         let mss = MediaSourceStream::new(Box::new(file), Default::default());

//         let hint = Hint::new();
//         let format_opts: FormatOptions = Default::default();
//         let metadata_opts: MetadataOptions = Default::default();
//         let decoder_opts: DecoderOptions = Default::default();

//         let probed = get_probe()
//             .format(&hint, mss, &format_opts, &metadata_opts)
//             .map_err(|e| e.to_string())?;

//         let mut format = probed.format;
//         let track = format.default_track().ok_or("No default track found")?;

//         let decoder = get_codecs()
//             .make(&track.codec_params, &decoder_opts)
//             .map_err(|e| e.to_string())?;

//         self.format_reader = Some(format);
//         self.decoder = Some(decoder);

//         // Assuming the decoder and format reader are set up correctly
//         if let Some(format_reader) = &mut self.format_reader {
//             if let Some(decoder) = &mut self.decoder {
//                 let source = SymphoniaSource::new(format_reader, decoder);
//                 self.output_stream_handle
//                     .play_raw(source.convert_samples())
//                     .map_err(|e| e.to_string())?;
//             }
//         }

//         Ok(())
//     }
// }

// struct SymphoniaSource<'a> {
//     format_reader: &'a mut dyn FormatReader,
//     decoder: &'a mut dyn symphonia::core::codecs::Decoder,
//     sample_rate: u32,
// }

// impl<'a> SymphoniaSource<'a> {
//     pub fn new(format_reader: &'a mut dyn FormatReader, decoder: &'a mut dyn symphonia::core::codecs::Decoder, sample_rate: u32) -> Self {
//         SymphoniaSource { format_reader, decoder, sample_rate }
//     }

//     // Function to decode the next packet and return the samples
//     fn decode_next_packet(&mut self) -> Option<Vec<f32>> {
//         loop {
//             match self.format_reader.next_packet() {
//                 Ok(packet) => {
//                     match self.decoder.decode(&packet) {
//                         Ok(audio_buf) => {
//                             // Assuming the audio buffer is in f32 format, adjust if necessary
//                             let samples = audio_buf.samples::<f32>().unwrap();
//                             return Some(samples.to_vec());
//                         },
//                         Err(_) => continue, // On decode error, try the next packet
//                     }
//                 },
//                 Err(_) => return None, // End of stream or error
//             }
//         }
//     }
// }

// impl<'a> Iterator for SymphoniaSource<'a> {
//     type Item = f32;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(buf) = &mut self.sample_buf {
//             if let Some(sample) = buf.pop() {
//                 return Some(sample);
//             }
//         }

//         // If the buffer is empty or not yet initialized, decode the next packet
//         if let Some(samples) = self.decode_next_packet() {
//             self.sample_buf = Some(samples.into_iter());
//             return self.sample_buf.as_mut()?.next();
//         }

//         None
//     }
// }

// impl<'a> Source for SymphoniaSource<'a> {
//     fn current_frame_len(&self) -> Option<usize> {
//         None // Variable frame length
//     }

//     fn channels(&self) -> u16 {
//         2 // Assuming stereo audio
//     }

//     fn sample_rate(&self) -> u32 {
//         self.sample_rate
//     }

//     fn total_duration(&self) -> Option<Duration> {
//         None // Stream duration is unknown
//     }
// }
