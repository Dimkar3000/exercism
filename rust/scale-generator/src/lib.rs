pub type Error = ();

#[derive(Clone)]
pub struct Scale {
    scale: Vec<Note>,
}

#[derive(Clone, PartialEq)]
pub enum Note {
    A,
    Ab,
    Asharp,
    B,
    Bb,
    C,
    Csharp,
    D,
    Db,
    Dsharp,
    E,
    Eb,
    F,
    Fsharp,
    G,
    Gb,
    Gsharp,
}

impl Note {
    pub fn new(input: &str) -> Self {
        return Note::from(input);
    }
}

impl std::convert::From<&str> for self::Note {
    fn from(note: &str) -> Self {
        match &note[..] {
            "Ab" => Note::Ab,
            "A" | "a" => Note::A,
            "A#" => Note::Asharp,
            "Bb" | "bb" => Note::Bb,
            "B" | "b" => Note::B,
            "C" | "c" => Note::C,
            "C#" | "c#" => Note::Csharp,
            "Db" => Note::Db,
            "D" | "d" => Note::D,
            "D#" => Note::Dsharp,
            "Eb" | "eb" => Note::Eb,
            "E" | "e" => Note::E,
            "F" | "f" => Note::F,
            "F#" | "f#" => Note::Fsharp,
            "Gb" => Note::Gb,
            "G" | "g" => Note::G,
            "G#" | "g#" => Note::Gsharp,
            _ => Note::A,
        }
    }
}

impl ToString for Note {
    fn to_string(&self) -> String {
        match self {
            Note::Ab => "Ab".to_string(),
            Note::A => "A".to_string(),
            Note::Asharp => "A#".to_string(),
            Note::Bb => "Bb".to_string(),
            Note::B => "B".to_string(),
            Note::C => "C".to_string(),
            Note::Csharp => "C#".to_string(),
            Note::Db => "Db".to_string(),
            Note::D => "D".to_string(),
            Note::Dsharp => "D#".to_string(),
            Note::Eb => "Eb".to_string(),
            Note::E => "E".to_string(),
            Note::F => "F".to_string(),
            Note::Fsharp => "F#".to_string(),
            Note::Gb => "Gb".to_string(),
            Note::G => "G".to_string(),
            Note::Gsharp => "G#".to_string(),
        }
    }
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let mut output: Vec<Note> = Vec::new();
        let mut int = intervals.chars();
        let c = Scale::chromatic(tonic);
        let c = c.unwrap().scale;
        let mut chromatic = c.into_iter();
        for _i in 0..intervals.len() {
            match int.next().unwrap() {
                'M' => {
                    output.push(chromatic.next().unwrap());
                    chromatic.next();
                }
                'm' => output.push(chromatic.next().unwrap()),
                'A' => {
                    output.push(chromatic.next().unwrap());
                    chromatic.next();
                    chromatic.next();
                }
                _ => (),
            }
        }
        Ok(Scale { scale: output })
    }

    pub fn sharp() -> Vec<Note> {
        [
            Note::A,
            Note::Asharp,
            Note::B,
            Note::C,
            Note::Csharp,
            Note::D,
            Note::Dsharp,
            Note::E,
            Note::F,
            Note::Fsharp,
            Note::G,
            Note::Gsharp,
        ]
        .to_vec()
    }

    pub fn flat() -> Vec<Note> {
        [
            Note::A,
            Note::Bb,
            Note::B,
            Note::C,
            Note::Db,
            Note::D,
            Note::Eb,
            Note::E,
            Note::F,
            Note::Gb,
            Note::G,
            Note::Ab,
        ]
        .to_vec()
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let sharp = Scale::sharp();
        let flat = Scale::flat();
        match tonic {
            "A" | "B" | "C" | "D" | "E" | "G" | "a" | "b" | "e" | "c#" | "d#" | "f#" | "g#" => {
                Ok(Scale {
                    scale: sharp
                        .clone()
                        .into_iter()
                        .cycle()
                        .skip(sharp.iter().position(|x| *x == Note::new(tonic)).unwrap())
                        .take(12)
                        .collect::<Vec<Note>>(),
                })
            }
            "Ab" | "Bb" | "Db" | "Eb" | "F" | "c" | "d" | "f" | "g" | "bb" | "eb" => Ok(Scale {
                scale: flat
                    .clone()
                    .into_iter()
                    .cycle()
                    .skip(flat.iter().position(|x| *x == Note::new(tonic)).unwrap())
                    .take(12)
                    .collect::<Vec<Note>>(),
            }),
            _ => Err(()),
        }
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.iter().map(ToString::to_string).collect()
    }
}
