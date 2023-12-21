use midly::Smf;
use std::env;
use std::fs::File;
use std::io::Read;

mod atm;
mod midi_parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please provide an output file path as an argument");
    }
    let file_path = args[1].clone();
    let out_path = &args[2];
    println!("File path is: {}", file_path);
    println!("Out file is: {}", out_path);

    let mut file = File::open(&file_path).expect("Unable to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Unable to read file");

    let smf = Smf::parse(&contents).unwrap();

    for (i, track) in smf.tracks.iter().enumerate() {
        for (event_index, event) in track.iter().enumerate() {
            //print_midi_event(event);
        }
    }
    let out = midi_parse::parse_midi_smf(smf);
    let json = serde_json::to_string(&out).expect("Failed to serialize ATMFile");

    //println!("{}", json);
    std::fs::write(out_path, &json).expect("Unable to write file");
}

fn print_midi_event(event: &midly::TrackEvent) {
    let event_kind = event.kind;
    match event_kind {
        midly::TrackEventKind::Meta(meta) => {
            print!("Meta event: {:?}", meta);
        }
        midly::TrackEventKind::Midi { channel, message } => {
            print!("Midi event: {:?} on channel {}", message, channel);
        }
        midly::TrackEventKind::SysEx(data) => {
            print!("SysEx event: {:?}", data);
        }
        midly::TrackEventKind::Escape(data) => {
            print!("Escape event: {:?}", data);
        }
    }
    println!(" delta: {:?}", event.delta)
}
