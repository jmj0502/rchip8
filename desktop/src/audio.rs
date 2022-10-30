use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::AudioSubsystem;

#[allow(dead_code)]
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 { 1.0 } else { -1.0 };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct AudioDeviceWrapper {
    device: AudioDevice<SquareWave>,
}

impl AudioDeviceWrapper {
    pub fn new(audio_subsystem: &AudioSubsystem) -> Self {
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: 240.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .expect("Couldn't open the sound playback!");

        Self { device }
    }

    pub fn beep(&self, should_beep: bool) {
        if should_beep {
            self.device.resume();
        } else {
            self.device.pause();
        }
    }
}
