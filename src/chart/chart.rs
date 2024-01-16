use regex::Regex;
use std::fs;

enum ParseBpmError {
    InvalidLine,
    InvalidBpm,
    InvalidTick,
}

pub struct Bpm {
    pub bpm: usize,
    pub tick: usize,
}

fn parse_bpm(bpm_regex: &Regex, line: &str) -> Result<Bpm, ParseBpmError> {
    let bpm = match bpm_regex.captures(line) {
        Some(v) => v,
        None => return Err(ParseBpmError::InvalidLine),
    };

    let (_, [raw_tick, kind, raw_bpm]) = bpm.extract();

    if kind != "B" {
        return Err(ParseBpmError::InvalidLine);
    }

    let bpm: usize = match raw_bpm.parse::<usize>() {
        Ok(parsed) => parsed,
        Err(_) => return Err(ParseBpmError::InvalidBpm),
    };

    let tick: usize = match raw_tick.parse::<usize>() {
        Ok(parsed) => parsed,
        Err(_) => return Err(ParseBpmError::InvalidTick),
    };

    Ok(Bpm { tick, bpm })
}

enum ParseNoteError {
    InvalidLine,
    InvalidColor,
    InvalidTick,
    UnsuportedNote,
    UnsuportedColor,
}

pub struct Note {
    pub color: usize,
    pub tick: usize,
}

fn parse_note(note_regex: &Regex, line: &str) -> Result<Note, ParseNoteError> {
    let note = match note_regex.captures(line) {
        Some(v) => v,
        None => return Err(ParseNoteError::InvalidLine),
    };

    let (_, [tick, kind, color, _length]) = note.extract();

    if kind != "N" {
        return Err(ParseNoteError::UnsuportedNote);
    }

    let color_parsed: usize = match color.parse::<usize>() {
        Ok(parsed) => parsed,
        Err(_) => return Err(ParseNoteError::InvalidColor),
    };

    if color_parsed > 4 {
        return Err(ParseNoteError::UnsuportedColor);
    }

    let tick_parsed: usize = match tick.parse::<usize>() {
        Ok(parsed) => parsed,
        Err(_) => return Err(ParseNoteError::InvalidTick),
    };

    Ok(Note {
        color: color_parsed,
        tick: tick_parsed,
    })
}

enum ParseMode {
    NOTE,
    BPM,
    NONE,
}

pub struct Chart {
    pub notes: Vec<Note>,
    pub bpms: Vec<Bpm>,
}

#[derive(Debug)]
pub enum ChartFromPathError {
    UnableToReadFile,
    InvalidRegex,
}

impl Chart {
    pub fn from_path(path: &str) -> Result<Chart, ChartFromPathError> {
        let file = match fs::read_to_string(path) {
            Ok(v) => v,
            Err(_) => return Err(ChartFromPathError::UnableToReadFile),
        };

        let title_regex = match Regex::new(r"\[(\w*)\]") {
            Ok(v) => v,
            Err(_) => return Err(ChartFromPathError::InvalidRegex),
        };

        let note_regex = match Regex::new(r"\s\s(\d*)\s=\s(\w)\s(\d)\s(\d+)") {
            Ok(v) => v,
            Err(_) => return Err(ChartFromPathError::InvalidRegex),
        };

        let bpm_regex = match Regex::new(r"\s\s(\d*)\s=\s(\w)\s(\d+)") {
            Ok(v) => v,
            Err(_) => return Err(ChartFromPathError::InvalidRegex),
        };

        let splited_file = file.split("\n");

        let mut parse_mode: ParseMode = ParseMode::NONE;

        let mut notes: Vec<Note> = vec![];
        let mut bpms: Vec<Bpm> = vec![];

        for line in splited_file.into_iter() {
            let title_cap = title_regex.captures(line);

            if let Some(title) = title_cap {
                let (_, [name]) = title.extract();

                if name == "ExpertSingle" {
                    parse_mode = ParseMode::NOTE;
                } else if name == "SyncTrack" {
                    parse_mode = ParseMode::BPM;
                } else {
                    parse_mode = ParseMode::NONE;
                }

                continue;
            }

            if line.starts_with("{") || line.starts_with("}") {
                continue;
            }

            match parse_mode {
                ParseMode::NONE => continue,
                ParseMode::BPM => {
                    let bpm = match parse_bpm(&bpm_regex, line) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };

                    bpms.push(bpm);
                }
                ParseMode::NOTE => {
                    let note = match parse_note(&note_regex, line) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };

                    notes.push(note);
                }
            };
        }

        Ok(Chart { notes, bpms })
    }
}
