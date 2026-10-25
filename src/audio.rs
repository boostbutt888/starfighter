//! Original orchestral-style score and synthesized effects, generated locally in Rust.
use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, PlaySoundParams, Sound};
use starfighter::Event;
use std::f32::consts::TAU;
fn wav(samples: &[f32], rate: u32) -> Vec<u8> {
    let size = samples.len() as u32 * 2;
    let mut b = Vec::with_capacity(44 + size as usize);
    b.extend_from_slice(b"RIFF");
    b.extend_from_slice(&(36 + size).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&rate.to_le_bytes());
    b.extend_from_slice(&(rate * 2).to_le_bytes());
    b.extend_from_slice(&2u16.to_le_bytes());
    b.extend_from_slice(&16u16.to_le_bytes());
    b.extend_from_slice(b"data");
    b.extend_from_slice(&size.to_le_bytes());
    for s in samples {
        b.extend_from_slice(&((s.tanh() * 29000.) as i16).to_le_bytes())
    }
    b
}
fn frequency(n: i32) -> f32 {
    440. * 2f32.powf((n as f32 - 69.) / 12.)
}
fn laser(enemy: bool, heavy: bool) -> Vec<u8> {
    let rate = 44100.;
    let duration = if heavy { 0.23 } else { 0.14 };
    let mut phase = 0.;
    let mut out = vec![];
    for i in 0..(rate * duration) as usize {
        let t = i as f32 / rate;
        let u = t / duration;
        let freq = (if enemy { 750. } else { 1550. }) * (1. - u * 0.85);
        phase += freq / rate * TAU;
        let v = (phase.sin() + 0.4 * (phase * 2.).sin() + 0.17 * (phase * 3.).sin())
            * (1. - u).powi(2)
            * (t * 700.).min(1.);
        out.push(v * 0.42)
    }
    wav(&out, rate as u32)
}
fn explosion(duration: f32, large: bool) -> Vec<u8> {
    let rate = 44100.;
    let (mut seed, mut low, mut rumble, mut phase) = (741u32, 0., 0., 0.);
    let mut out = vec![];
    for i in 0..(duration * rate) as usize {
        let t = i as f32 / rate;
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = (seed >> 16) as f32 / 32768. - 1.;
        let cutoff = 0.09 * (-t * 2.5).exp() + 0.006;
        low += (noise - low) * cutoff;
        rumble += (low - rumble) * 0.025;
        phase += (30. + 65. * (-t * 5.).exp()) / rate * TAU;
        let env = (1. - (-t * 150.).exp()) * (-t * if large { 1.3 } else { 3. }).exp();
        let sub = phase.sin() * 0.6 + (phase * 2.).sin() * 0.15;
        let v = (sub + low * 2.1 + rumble * 3.) * env;
        let edge = ((duration - t) / 0.15).min(1.);
        out.push(v * edge)
    }
    wav(&out, rate as u32)
}
const MELODY: [[i32; 8]; 16] = [
    [74, 0, 69, 72, 74, 0, 77, 76],
    [74, 72, 69, 0, 67, 69, 72, 0],
    [70, 0, 65, 69, 70, 0, 74, 72],
    [70, 69, 65, 0, 64, 65, 69, 0],
    [72, 0, 67, 70, 72, 0, 76, 74],
    [72, 70, 67, 0, 65, 67, 70, 0],
    [69, 0, 73, 76, 81, 0, 79, 76],
    [73, 0, 76, 0, 69, 0, 0, 0],
    [77, 0, 74, 77, 81, 0, 79, 77],
    [76, 74, 72, 0, 69, 72, 74, 0],
    [74, 0, 70, 74, 77, 0, 76, 74],
    [72, 70, 69, 0, 65, 69, 70, 0],
    [76, 0, 72, 76, 79, 0, 81, 79],
    [77, 76, 74, 0, 72, 74, 76, 0],
    [73, 0, 76, 79, 81, 0, 76, 73],
    [74, 0, 0, 0, 69, 72, 73, 0],
];
const CHORDS: [[i32; 4]; 8] = [
    [50, 57, 62, 65],
    [50, 57, 60, 65],
    [46, 53, 58, 62],
    [46, 53, 57, 62],
    [48, 55, 60, 64],
    [48, 55, 58, 64],
    [45, 52, 57, 61],
    [45, 52, 55, 61],
];
// A synthesized main-theme arrangement, with a separate minor-key tension cue.
const MAIN_THEME: [[i32; 8]; 8] = [
    [70, 70, 70, 77, 77, 77, 82, 82],
    [80, 79, 77, 89, 89, 89, 82, 82],
    [80, 79, 77, 89, 89, 89, 82, 82],
    [80, 79, 80, 77, 77, 77, 70, 70],
    [70, 70, 70, 77, 77, 77, 82, 82],
    [80, 79, 77, 89, 89, 89, 82, 82],
    [80, 79, 77, 89, 89, 89, 82, 82],
    [80, 79, 80, 77, 77, 77, 0, 0],
];
fn score(boss: bool) -> Vec<u8> {
    let rate = 22050.;
    let unit = 60. / if boss { 138. } else { 118. } / 2.;
    let duration = unit * 128.;
    let mut out = Vec::with_capacity((duration * rate) as usize);
    let (mut seed, mut oldnoise) = (73u32, 0.);
    for i in 0..(duration * rate) as usize {
        let t = i as f32 / rate;
        let eighth = t / unit;
        let step = eighth as usize;
        let bar = (step / 8) % 16;
        let beat = step % 8;
        let nt = eighth.fract();
        let note = if boss {
            MELODY[bar][beat] - 12
        } else {
            MAIN_THEME[bar % 8][beat]
        };
        let chord = if boss {
            CHORDS[bar % 8]
        } else {
            [46, 53, 58, 62]
        };
        let mut v = 0.;
        if note > 0 {
            let f = frequency(note);
            let p = TAU * f * t;
            let env = (nt * 12.).min(1.) * ((1. - nt) / 0.25).min(1.);
            v += (p.sin() + 0.42 * (p * 2.).sin() + 0.23 * (p * 3.).sin() + 0.08 * (p * 5.).sin())
                * 0.15
                * env;
            v += (p * 0.5).sin() * 0.06 * env
        }
        for n in chord {
            let f = frequency(n);
            let p = TAU * f * t;
            v += (p.sin() + 0.65 * (p * 1.003).sin() + 0.25 * (p * 2.).sin()) * 0.021
        }
        let arp = chord[[0, 2, 1, 3, 2, 1, 3, 2][beat]] + 12;
        let p = TAU * frequency(arp) * t;
        v +=
            (p.sin() + 0.4 * (p * 2.).sin()) * (1. - nt).powi(2) * if boss { 0.075 } else { 0.045 };
        let bt = (eighth / 2.).fract();
        let bass = TAU * frequency(chord[0] - 12) * t;
        v += bass.sin() * 0.12 * (1. - bt * 0.5);
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = (seed >> 16) as f32 / 32768. - 1.;
        if beat == 0 || beat == 4 || (boss && beat == 6) {
            let d = nt * unit;
            let drum = TAU * (72. * d - 32. * d * d);
            v += drum.sin() * (-d * 12.).exp() * 0.28
        }
        if beat == 2 || beat == 6 {
            v += (noise - oldnoise * 0.5) * (1. - nt).powi(5) * 0.11
        }
        if nt < 0.07 {
            v += (noise - oldnoise) * 0.025
        }
        oldnoise = noise;
        let fade = (t / 0.035).min(1.) * ((duration - t) / 0.05).min(1.);
        out.push(v * fade)
    } // short stereo-like early reflections in a mono portable WAV
    let dry = out.clone();
    for (delay, gain) in [(0.07, 0.13), (0.13, 0.08), (0.21, 0.045)] {
        let d = (delay * rate) as usize;
        for i in d..out.len() {
            out[i] += dry[i - d] * gain
        }
    }
    wav(&out, rate as u32)
}
fn pickup() -> Vec<u8> {
    let rate = 22050.;
    let notes = [74, 77, 81, 86];
    let mut out = vec![];
    for i in 0..11025 {
        let t = i as f32 / rate;
        let note = notes[((t / 0.09) as usize).min(3)];
        let p = TAU * frequency(note) * t;
        out.push(p.sin() * 0.28 * (-t * 4.).exp())
    }
    wav(&out, rate as u32)
}
async fn sound(bytes: Vec<u8>) -> Option<Sound> {
    load_sound_from_bytes(&bytes).await.ok()
}
fn play(s: &Option<Sound>, volume: f32, looped: bool) {
    if let Some(s) = s {
        play_sound(s, PlaySoundParams { looped, volume })
    }
}
pub struct Audio {
    flight: Option<Sound>,
    boss: Option<Sound>,
    laser: Option<Sound>,
    enemy: Option<Sound>,
    heavy: Option<Sound>,
    blast: Option<Sound>,
    deep: Option<Sound>,
    pickup: Option<Sound>,
    pub music: bool,
    pub effects: bool,
    playing: Option<bool>,
}
impl Audio {
    pub async fn new() -> Self {
        Self {
            flight: sound(score(false)).await,
            boss: sound(score(true)).await,
            laser: sound(laser(false, false)).await,
            enemy: sound(laser(true, false)).await,
            heavy: sound(laser(true, true)).await,
            blast: sound(explosion(1.2, false)).await,
            deep: sound(explosion(3., true)).await,
            pickup: sound(pickup()).await,
            music: true,
            effects: true,
            playing: None,
        }
    }
    pub fn update(&mut self, events: &[Event], boss: bool, active: bool) {
        let wanted = if self.music && active {
            Some(boss)
        } else {
            None
        };
        if self.playing != wanted {
            for s in [&self.flight, &self.boss].into_iter().flatten() {
                stop_sound(s)
            }
            if let Some(b) = wanted {
                play(if b { &self.boss } else { &self.flight }, 0.35, true)
            }
            self.playing = wanted
        }
        if !self.effects || !active {
            return;
        }
        let mut enemy_count = 0;
        for e in events {
            match *e {
                Event::Laser(enemy, heavy) => {
                    if enemy {
                        enemy_count += 1;
                        if enemy_count > 2 {
                            continue;
                        }
                    }
                    play(
                        if heavy {
                            &self.heavy
                        } else if enemy {
                            &self.enemy
                        } else {
                            &self.laser
                        },
                        if enemy { 0.13 } else { 0.25 },
                        false,
                    )
                }
                Event::Explosion(size) => play(
                    if size > 1.4 { &self.deep } else { &self.blast },
                    if size > 1.4 { 0.65 } else { 0.35 },
                    false,
                ),
                Event::Pickup | Event::Victory => play(&self.pickup, 0.4, false),
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_audio_has_valid_pcm_headers_and_sub_bass() {
        for data in [laser(false, false), explosion(1., false), pickup()] {
            assert_eq!(&data[0..4], b"RIFF");
            assert_eq!(&data[8..12], b"WAVE");
            assert!(data.len() > 1000)
        }
    }
}
