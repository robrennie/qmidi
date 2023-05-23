use midi_file::core::{Channel, Clocks, DurationName, GeneralMidi, NoteNumber, Velocity, PitchBendValue};
use midi_file::file::{QuartersPerMinute, Track};
use midi_file::MidiFile;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// pattern step
pub struct PatternStep {
    pub is_rest: bool,
    pub duration: u32,
    pub velocity: Velocity
}

// durations
pub const WHOLE: u32 = 4096;
pub const HALF: u32 = 2048;
pub const QUARTER: u32 = 1024;
pub const EIGHTH: u32 = QUARTER / 2;
pub const DOTTED_QUARTER: u32 = QUARTER + EIGHTH;

// pitch bends
const PITCHBEND_NONE: u16 = 8192;
const PITCHBEND_SEMITONE: u16 = 4096;

// some arbitrary velocity
const V: Velocity = Velocity::new(64);

// channel zero (displayed as channel 1 in any sequencer UI)
const CH: Channel = Channel::new(0);

pub fn create_qsong(input_file: &str, output_file: &str, tempo: u8, timesig_num: u8, pattern: &Vec<PatternStep>) {

    let path_in = Path::new(input_file);
    let song_raw = read_input(path_in);

    let path_out = Path::new(output_file);
    let mut mfile = MidiFile::new();
    let mut track = Track::default();

    track.set_name("Quantum").unwrap();
    track.set_instrument_name("Piano").unwrap();
    track.set_general_midi(CH, GeneralMidi::AcousticGrandPiano).unwrap();

    // set time signature and tempo
    track
        .push_time_signature(0, timesig_num, DurationName::Quarter, Clocks::Quarter)
        .unwrap();
    track.push_tempo(0, QuartersPerMinute::new(tempo)).unwrap();

    let mut pattern_step = 0;
    for raw_note in song_raw {
        let (note_number, pitch_bend) = find_closest_note_with_pitch_bend(raw_note);

        let mut delta_time: u32 = 0;
        let mut duration: u32 = QUARTER;
        let mut velocity = V;
        if pattern.len() > 0 {
            while pattern[pattern_step].is_rest {
                delta_time += pattern[pattern_step].duration;
                pattern_step += 1;
                if pattern_step == pattern.len() {
                    pattern_step = 0;
                }
            }
            duration = pattern[pattern_step].duration;
            velocity = pattern[pattern_step].velocity;
            pattern_step += 1;
            if pattern_step == pattern.len() {
                pattern_step = 0;
            }
        }
        track.push_pitch_bend(delta_time, CH, PitchBendValue::new(pitch_bend)).unwrap();
        track.push_note_on(0, CH, note_number, velocity).unwrap();
        track.push_note_off(duration, CH, note_number, velocity).unwrap();
    }

    mfile.push_track(track).unwrap();
    mfile.save(path_out).unwrap();
}

pub fn find_closest_note_with_pitch_bend(raw_note: u32) -> (NoteNumber, u16) {
    let f = raw_note as f32;
    let nf = ( 12.0 * (f / 220.0).log10() / (2.0f32).log(10.0) ) + 57.01;
    let nn = nf as u8;
    if nn > 127 {
        return (NoteNumber::new(127), PITCHBEND_NONE);
    }
    let pb = (nf - (nn as f32)) * (PITCHBEND_SEMITONE as f32);

    (NoteNumber::new(nn), PITCHBEND_NONE + (pb as u16))
}

pub fn read_input(path: &Path) -> Vec<u32> {
    let mut res = Vec::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                res.push(ip.trim().parse::<u32>().unwrap());
            }
        }
    }
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}