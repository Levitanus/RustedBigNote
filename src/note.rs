use druid::widget::SvgData;

use tracing::error;

// mod note;

const LINES_AMOUTN: usize = 24;
const NOTE_NAMES: [&str; 12] = ["C", "C", "D", "D", "E", "F", "F", "G", "G", "A", "A", "B"];
const NOTE_LINES: [NoteLine; 24] = [
    NoteLine::new(0.0, false), // C
    NoteLine::new(0.0, true),  // C#
    NoteLine::new(0.5, false), // D
    NoteLine::new(0.5, true),  // D#
    NoteLine::new(1.0, false), // E
    NoteLine::new(1.5, false), // F
    NoteLine::new(1.5, true),  // F#
    NoteLine::new(2.0, false), // G
    NoteLine::new(2.0, true),  // G#
    NoteLine::new(2.5, false), // A
    NoteLine::new(2.5, true),  // A#
    NoteLine::new(3.0, false), // B
    NoteLine::new(3.5, false), // C
    NoteLine::new(3.5, true),  // C#
    NoteLine::new(4.0, false), // D
    NoteLine::new(4.0, true),  // D#
    NoteLine::new(4.5, false), // E
    NoteLine::new(5.0, false), // F
    NoteLine::new(5.0, true),  // F#
    NoteLine::new(5.5, false), // G
    NoteLine::new(5.5, true),  // G#
    NoteLine::new(6.0, false), // A
    NoteLine::new(6.0, true),  // A#
    NoteLine::new(6.5, false), // B
];

#[test]
fn test_note_line() {
    assert_eq!(
        NOTE_LINES[0].from_alteration(NoteAlt::Sharp),
        (0.0, NoteAlt::White)
    );
    assert_eq!(
        NOTE_LINES[1].from_alteration(NoteAlt::Sharp),
        (0.0, NoteAlt::Sharp)
    );
    assert_eq!(
        NOTE_LINES[1].from_alteration(NoteAlt::Flat),
        (0.5, NoteAlt::Flat)
    );
    assert_eq!(
        NOTE_LINES[23].from_alteration(NoteAlt::Flat),
        (6.5, NoteAlt::White)
    );
    assert_eq!(
        NOTE_LINES[22].from_alteration(NoteAlt::Flat),
        (6.5, NoteAlt::Flat)
    );
}

#[test]
fn test_note() {
    let c3 = Note { midi_nr: 60 };
    assert_eq!(c3.name(NoteAlt::Sharp), String::from("C3"));
    assert_eq!(c3.alteration(NoteAlt::Sharp), NoteAlt::White);
    assert_eq!(c3.line(NoteAlt::Sharp), 17.5);

    let fis3 = Note { midi_nr: 66 };
    assert_eq!(fis3.name(NoteAlt::Sharp), String::from("F#3"));
    assert_eq!(fis3.alteration(NoteAlt::Sharp), NoteAlt::Sharp);
    assert_eq!(fis3.line(NoteAlt::Sharp), 19.0);
    assert_eq!(fis3.name(NoteAlt::Flat), String::from("Gb3"));
    assert_eq!(fis3.alteration(NoteAlt::Flat), NoteAlt::Flat);
    assert_eq!(fis3.line(NoteAlt::Flat), 19.5);
}

#[derive(Clone, Debug, PartialEq)]
pub enum NoteAlt {
    White,
    Sharp,
    Flat,
}
impl NoteAlt {
    pub fn svgdata(&self) -> Option<SvgData> {
        let file: Result<SvgData, Box<dyn std::error::Error + 'static>>;
        match *self {
            NoteAlt::White => return None,
            NoteAlt::Sharp => file = include_str!("../assets/sharp.svg").parse::<SvgData>(),
            NoteAlt::Flat => file = include_str!("../assets/flat.svg").parse::<SvgData>(),
        }
        match file {
            Ok(svg) => Some(svg),
            Err(err) => {
                error!("{}", err);
                error!("Using an empty SVG instead.");
                Some(SvgData::default())
            }
        }
    }
}
impl std::fmt::Display for NoteAlt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            NoteAlt::White => write!(f, ""),
            NoteAlt::Sharp => write!(f, "#"),
            NoteAlt::Flat => write!(f, "b"),
        }
    }
}

#[derive(Debug)]
pub struct NoteLine {
    root: f32,
    alterated: bool,
}
impl NoteLine {
    const fn new(root: f32, alterated: bool) -> Self {
        NoteLine {
            root: root,
            alterated: alterated,
        }
    }
    fn from_alteration(&self, alteration: NoteAlt) -> (f32, NoteAlt) {
        if self.alterated == false {
            return (self.root, NoteAlt::White);
        } else if NoteAlt::Sharp == alteration {
            return (self.root, NoteAlt::Sharp);
        } else {
            return (self.root + 0.5, NoteAlt::Flat);
        }
    }
}

pub struct Note {
    midi_nr: u8,
}
impl Note {
    pub fn spec(&self, alteration: NoteAlt) -> (f32, NoteAlt, String) {
        let midi_nr = self.midi_nr as usize;
        let modulo = midi_nr / LINES_AMOUTN;
        let remainder = midi_nr % LINES_AMOUTN;
        let (line, alt) = &NOTE_LINES[remainder].from_alteration(alteration);
        let line_full = line + (modulo * 7) as f32;
        let octave = (midi_nr as usize / 12) - 2;
        let mut note_name: &str;
        if alt == &NoteAlt::Sharp || alt == &NoteAlt::White {
            note_name = &NOTE_NAMES[midi_nr % 12];
        } else {
            note_name = &NOTE_NAMES[(midi_nr % 12) + 1];
        }
        let name = format!("{}{}{}", note_name, alt, octave);
        (line_full, alt.clone(), name)
    }
    pub fn line(&self, alteration: NoteAlt) -> f32 {
        let (line, _alt, _name) = self.spec(alteration);
        line
    }
    pub fn alteration(&self, alteration: NoteAlt) -> NoteAlt {
        let (_line, alt, _name) = self.spec(alteration);
        alt
    }
    pub fn name(&self, alteration: NoteAlt) -> String {
        let (_line, _alt, name) = self.spec(alteration);
        return name;
    }
}
