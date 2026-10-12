use std::{fmt::Display, time::Duration};

pub struct TypingTestSpeed {
    pub net: f32,
    pub raw: f32,
    pub unit: TypingSpeedUnit,
}

impl TypingTestSpeed {
    pub fn new(strokes: u32, errors: u32, time: Duration, unit: TypingSpeedUnit) -> Self {
        let minutes = time.as_secs_f32() / 60.0;
        let net = match unit {
            TypingSpeedUnit::WordsPerMinute => (strokes - errors) as f32 / 5.0 / minutes,
        };

        let raw = match unit {
            TypingSpeedUnit::WordsPerMinute => strokes as f32 / 5.0 / minutes,
        };

        Self { net, raw, unit }
    }
}

pub enum TypingSpeedUnit {
    WordsPerMinute,
}

impl Display for TypingSpeedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypingSpeedUnit::WordsPerMinute => write!(f, "wpm"),
        }
    }
}
