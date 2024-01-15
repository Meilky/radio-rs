use regex::Regex;
use std::fs;

pub struct Note {
    pub color: usize,
    pub tick: usize,
}

pub struct Bpm {
    pub bpm: usize,
    pub tick: usize,
}

pub struct Chart {
    pub notes: Vec<Note>,
    pub bpms: Vec<Bpm>,
}

enum ParseNoteError {
    InvalidLine,
    InvalidColor,
    InvalidTick,
    UnsuportedNote,
    UnsuportedColor,
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

        let splited_file = file.split("\n");

        let mut is_notes: bool = false;

        let mut notes: Vec<Note> = vec![];

        for line in splited_file.into_iter() {
            let title_cap = title_regex.captures(line);

            if let Some(title) = title_cap {
                let (_, [name]) = title.extract();

                if name == "ExpertSingle" {
                    is_notes = true;
                } else {
                    is_notes = false;
                }

                continue;
            }

            if !is_notes {
                continue;
            }

            if line.starts_with("{") || line.starts_with("}") {
                continue;
            }

            let note = match parse_note(&note_regex, line) {
                Ok(v) => v,
                Err(_) => continue,
            };

            notes.push(note);
        }

        Ok(Chart {
            notes,
            bpms: vec![],
        })
    }
}
