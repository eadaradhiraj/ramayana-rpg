use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};

pub struct SoundEffects {
    pub shoot: Sound,
    pub hit: Sound,
    pub pin: Sound,
    pub ripple: Sound,
    pub boulder: Sound,
    pub smash: Sound,
    pub roar: Sound,
    pub hurt: Sound,
    pub victory: Sound,
    pub game_over: Sound,
    // Level 2 Astras & Altar
    pub wind: Sound,
    pub homing: Sound,
    pub fire: Sound,
    pub altar_hiss: Sound,
}

impl SoundEffects {
    pub async fn init() -> Self {
        let shoot = load_sound_from_bytes(&generate_shoot_wav()).await.unwrap();
        let hit = load_sound_from_bytes(&generate_hit_wav()).await.unwrap();
        let pin = load_sound_from_bytes(&generate_pin_wav()).await.unwrap();
        let ripple = load_sound_from_bytes(&generate_ripple_wav()).await.unwrap();
        let boulder = load_sound_from_bytes(&generate_boulder_wav()).await.unwrap();
        let smash = load_sound_from_bytes(&generate_smash_wav()).await.unwrap();
        let roar = load_sound_from_bytes(&generate_roar_wav()).await.unwrap();
        let hurt = load_sound_from_bytes(&generate_hurt_wav()).await.unwrap();
        let victory = load_sound_from_bytes(&generate_victory_wav()).await.unwrap();
        let game_over = load_sound_from_bytes(&generate_game_over_wav()).await.unwrap();
        let wind = load_sound_from_bytes(&generate_wind_wav()).await.unwrap();
        let homing = load_sound_from_bytes(&generate_homing_wav()).await.unwrap();
        let fire = load_sound_from_bytes(&generate_fire_wav()).await.unwrap();
        let altar_hiss = load_sound_from_bytes(&generate_altar_hiss_wav()).await.unwrap();

        Self {
            shoot,
            hit,
            pin,
            ripple,
            boulder,
            smash,
            roar,
            hurt,
            victory,
            game_over,
            wind,
            homing,
            fire,
            altar_hiss,
        }
    }

    pub fn play(&self, sound: &Sound, volume: f32) {
        play_sound(
            sound,
            PlaySoundParams {
                looped: false,
                volume,
            },
        );
    }
}

fn build_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(44 + samples.len() * 2);
    let num_samples = samples.len() as u32;
    let byte_rate = sample_rate * 2;
    let data_size = num_samples * 2;

    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_size).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_size.to_le_bytes());

    for &s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

fn generate_shoot_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.12;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    let mut phase = 0.0f32;
    for i in 0..n {
        let t = i as f32 / n as f32;
        let freq = 850.0 * (1.0 - t * 0.75);
        phase += freq / rate as f32;
        let env = (1.0 - t).powf(1.8);
        let wave = ((phase % 1.0) * 2.0 - 1.0).abs() * 2.0 - 1.0;
        let s = (wave * env * 12000.0) as i16;
        samples.push(s);
    }
    build_wav(&samples, rate)
}

fn generate_hit_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.15;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    let mut rng: u32 = 12345;
    for i in 0..n {
        let t = i as f32 / n as f32;
        let env = (1.0 - t).powf(2.5);
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((rng >> 16) as f32 / 32768.0) - 1.0;
        let thud = ((i as f32 * 0.08).sin()) * (1.0 - t);
        let sample = (noise * 0.6 + thud * 0.4) * env * 15000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

fn generate_pin_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.35;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    for i in 0..n {
        let t = i as f32 / rate as f32;
        let progress = i as f32 / n as f32;
        let env = (1.0 - progress).powf(1.5);
        let s1 = (2.0 * std::f32::consts::PI * 880.0 * t).sin();
        let s2 = (2.0 * std::f32::consts::PI * 1320.0 * t).sin() * 0.5;
        let sample = (s1 + s2) * env * 14000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

fn generate_ripple_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.45;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    for i in 0..n {
        let t = i as f32 / rate as f32;
        let progress = i as f32 / n as f32;
        let env = (progress * std::f32::consts::PI).sin().powf(1.2);
        let tremolo = 1.0 + 0.3 * (2.0 * std::f32::consts::PI * 12.0 * t).sin();
        let freq = 520.0 + 60.0 * (1.0 - progress);
        let wave = (2.0 * std::f32::consts::PI * freq * t).sin();
        let sample = wave * tremolo * env * 9000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

fn generate_boulder_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.22;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    for i in 0..n {
        let t = i as f32 / n as f32;
        let env = (t * std::f32::consts::PI).sin();
        let freq = 180.0 * (1.0 - t * 0.5);
        let wave = (2.0 * std::f32::consts::PI * freq * (i as f32 / rate as f32)).sin();
        let sample = wave * env * 16000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

fn generate_smash_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.3;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    let mut rng: u32 = 98765;
    for i in 0..n {
        let t = i as f32 / n as f32;
        let env = (1.0 - t).powf(1.8);
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((rng >> 16) as f32 / 32768.0) - 1.0;
        let boom = ((i as f32 * 0.035).sin()) * (1.0 - t);
        let sample = (noise * 0.7 + boom * 0.3) * env * 17000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

fn generate_roar_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.65;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    let mut phase = 0.0f32;
    for i in 0..n {
        let t = i as f32 / n as f32;
        let env = if t < 0.2 { t / 0.2 } else { (1.0 - t).powf(1.2) };
        let freq = 110.0 - t * 35.0;
        phase += freq / rate as f32;
        let saw = (phase % 1.0) * 2.0 - 1.0;
        let sample = (saw + (saw * 2.5).clamp(-1.0, 1.0)) * 0.5 * env * 16000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

fn generate_hurt_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.2;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    for i in 0..n {
        let t = i as f32 / n as f32;
        let env = (1.0 - t).powf(1.5);
        let freq = 260.0 - t * 140.0;
        let square = if ((i as f32 / rate as f32 * freq) % 1.0) < 0.5 { 1.0 } else { -1.0 };
        let sample = square * env * 13000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

fn generate_victory_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 1.0;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    let notes = [523.25, 659.25, 783.99, 1046.50];
    for i in 0..n {
        let t = i as f32 / rate as f32;
        let note_idx = ((i as f32 / n as f32) * 4.0).min(3.0) as usize;
        let freq = notes[note_idx];
        let note_time = t % 0.25;
        let env = (1.0 - note_time / 0.25).powf(0.8);
        let wave = (2.0 * std::f32::consts::PI * freq * t).sin();
        let sample = wave * env * 14000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

fn generate_game_over_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.8;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    let notes = [392.00, 349.23, 293.66];
    for i in 0..n {
        let t = i as f32 / rate as f32;
        let note_idx = ((i as f32 / n as f32) * 3.0).min(2.0) as usize;
        let freq = notes[note_idx];
        let note_time = t % (0.8 / 3.0);
        let env = (1.0 - note_time / (0.8 / 3.0)).powf(1.3);
        let wave = (2.0 * std::f32::consts::PI * freq * t).sin();
        let sample = wave * env * 14000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

// Level 2: Vayavyastra (Cyclone Wind Gust)
fn generate_wind_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.4;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    let mut rng: u32 = 45678;
    for i in 0..n {
        let t = i as f32 / n as f32;
        let env = (t * std::f32::consts::PI).sin().powf(0.9);
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((rng >> 16) as f32 / 32768.0) - 1.0;
        let freq = 340.0 + (t * 4.0).sin() * 80.0;
        let sine = (2.0 * std::f32::consts::PI * freq * (i as f32 / rate as f32)).sin();
        let sample = (noise * 0.7 + sine * 0.3) * env * 16000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

// Level 2: Manavastra (Homing Celestial Strike)
fn generate_homing_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.45;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    for i in 0..n {
        let t = i as f32 / n as f32;
        let env = (1.0 - t).powf(0.9);
        let freq = 440.0 + t * 660.0; // rising pitch
        let s1 = (2.0 * std::f32::consts::PI * freq * (i as f32 / rate as f32)).sin();
        let s2 = (2.0 * std::f32::consts::PI * (freq * 1.5) * (i as f32 / rate as f32)).sin() * 0.4;
        let sample = (s1 + s2) * env * 15000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

// Level 2: Agneyastra (Holy Fire Combustion)
fn generate_fire_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.38;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    let mut rng: u32 = 78912;
    for i in 0..n {
        let t = i as f32 / n as f32;
        let env = (1.0 - t).powf(1.6);
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((rng >> 16) as f32 / 32768.0) - 1.0;
        let low_rumble = ((i as f32 * 0.04).sin()) * 0.8;
        let sample = (noise * 0.6 + low_rumble * 0.4) * env * 18000.0;
        samples.push(sample as i16);
    }
    build_wav(&samples, rate)
}

// Level 2: Altar Defiled Hiss
fn generate_altar_hiss_wav() -> Vec<u8> {
    let rate = 22050;
    let dur = 0.35;
    let n = (rate as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);

    let mut rng: u32 = 34567;
    for i in 0..n {
        let t = i as f32 / n as f32;
        let env = (1.0 - t).powf(2.0);
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((rng >> 16) as f32 / 32768.0) - 1.0;
        let hiss = (noise * (0.8 + 0.2 * (t * 30.0).sin())) * env * 14000.0;
        samples.push(hiss as i16);
    }
    build_wav(&samples, rate)
}
