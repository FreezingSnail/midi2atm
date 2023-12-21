use crate::atm::{self, new_empty_atm, ATMFile};
use midly::Smf;

pub fn parse_midi_smf(midi: Smf) -> ATMFile {
    let mut out = new_empty_atm();
    for (i, track) in midi.tracks.iter().enumerate() {
        let mut track_notes: Vec<atm::Note> = Vec::new();

        for (_, event) in track.iter().enumerate() {
            match &event.kind {
                midly::TrackEventKind::Midi { message, .. } => match message {
                    midly::MidiMessage::NoteOn { .. } => {
                        let note_block = note_on_to_atm_notes(event);
                        track_notes.extend(note_block);
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }
        let chunks: Vec<_> = track_notes.chunks(32).map(|chunk| chunk.to_vec()).collect();
        for (j, chunk) in chunks.iter().enumerate() {
            let channel = atm::ATMChannels {
                ticks: chunk.len() as i32,
                color: atm::Color {
                    hex: "#ff7e00".to_string(),
                    rgb: atm::RGB {
                        r: 255,
                        g: 126,
                        b: 0,
                    },
                },
                id: (j + (i * j)) as i32,
                name: "test".to_string(),
                tune: atm::SoundType::tune,
                notes: chunk.clone(),
                editorId: (j + (i * j)) as i32,
            };
            out.channels[i].push(channel);
            let channel = atm::ATMChannels {
                ticks: chunk.len() as i32,
                color: atm::Color {
                    hex: "#ff7e00".to_string(),
                    rgb: atm::RGB {
                        r: 255,
                        g: 126,
                        b: 0,
                    },
                },
                id: (j + (i * j)) as i32,
                name: "test".to_string(),
                tune: atm::SoundType::tune,
                notes: chunk.clone(),
                editorId: 0,
            };
            out.patterns.push(channel);
        }

        //print!("Track notes: {:?}", track_notes)
    }
    return out;
}

fn note_on_to_atm_notes(note: &midly::TrackEvent) -> Vec<atm::Note> {
    let mut notes = Vec::new();
    let mut new_note = atm::Note {
        active: 23,
        noteStr: "".to_string(),
    };
    let delta: u32 = note.delta.into();
    match note.kind {
        midly::TrackEventKind::Midi { message, .. } => match message {
            midly::MidiMessage::NoteOn {
                key: note_number, ..
            } => {
                // A4 is 69 in midi, but A4 is 10 in ATM
                //needs to be a conversion
                //can only use notes c4-d9
                let mut tmp: u8 = note_number.into();
                tmp += 30;
                if tmp < 59 {
                    tmp = 59;
                }
                tmp -= 59;
                new_note.active = tmp.into();
                new_note.noteStr = midi_note_to_letter(note_number.into()); //   note_number.to_string();
            }
            _ => {}
        },
        _ => {}
    }
    notes.push(new_note);
    // need to convert delta somhow.
    //think it might be based on tempo
    //i.e. tempo / duration rounded up to 30?

    // for _ in 0..delta {
    //     let new_note = atm::Note {
    //         active: 23,
    //         noteStr: "...".to_string(),
    //     };
    //     notes.push(new_note);
    // }

    return notes;
}

fn midi_note_to_letter(note: u8) -> String {
    let notes = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let octave = note / 12 - 1; // MIDI note 0 is C-1
    let note = notes[(note % 12) as usize];
    format!("{}{}", note, octave)
}
