use serde::{Deserialize, Serialize};
//use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ATMFile {
    pub fx: ATMFX,
    pub channels: [Vec<ATMChannels>; 4],
    pub patterns: Vec<ATMChannels>,
    pub meta: ATMMeta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ATMFX {
    pub enabled: bool,
    pub status: FXStatus,
    pub channel: [FXChannel; 4],
    pub pattern: FXPattern,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FXStatus {
    pub fx_type: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct FXChannel {
    pub flags: i32,
    pub fx: FXFX,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct FXFX {
    pub id: i32,
    pub pattern: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FXPattern {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ATMChannels {
    pub ticks: i32,
    pub color: Color,
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub tune: SoundType,
    pub notes: Vec<Note>,
    pub editorId: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Color {
    pub hex: String,
    pub rgb: RGB,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct RGB {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum SoundType {
    tune,
    DRUM,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub active: i32,
    pub noteStr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ATMMeta {
    pub song_name: String,
    pub repeat: bool,
}

pub fn new_empty_atm() -> ATMFile {
    ATMFile {
        fx: ATMFX {
            enabled: false,
            status: FXStatus {
                fx_type: "".to_string(),
                id: 0,
            },
            channel: [FXChannel {
                flags: 0,
                fx: FXFX { id: 0, pattern: 0 },
            }; 4],
            pattern: FXPattern {},
        },
        channels: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        patterns: Vec::new(),
        meta: ATMMeta {
            song_name: "test".to_string(),
            repeat: false,
        },
    }
}
