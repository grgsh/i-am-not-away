use std::{
    f64::consts::E,
    thread,
    time::{Duration, Instant},
};

pub struct Animator {
    pub config: AnimatorConfig,
}
pub struct AnimatorConfig {
    pub target_fps: u32,
    pub speed: u32,
}

impl Animator {
    pub fn new(config: Option<AnimatorConfig>) -> Self {
        if let Some(config) = config {
            Self { config }
        } else {
            Self {
                config: AnimatorConfig {
                    target_fps: 1,
                    speed: 100,
                },
            }
        }
    }

    fn run<F>(&self, mut on_tick: F)
    where
        F: FnMut(),
    {
        let target_fps = self.config.target_fps;
        let speed = self.config.speed as f64 / 100_f64;
        let actual_target_fps: u8 = ((target_fps as f64 * speed).floor() as f64) as u8;
        println!("{} {}", actual_target_fps, speed);
        let target_sleep_time = Duration::from_micros(1_000_000 / actual_target_fps.max(1) as u64);

        loop {
            on_tick();

            let frame_start = Instant::now();
            let frame_time = frame_start.elapsed();
            if frame_time < target_sleep_time {
                thread::sleep(target_sleep_time - frame_time);
            }
        }
    }

    // TODO: use the power
    pub fn oscillate<F>(&self, min: u8, max: u8, power: u8, mut on_tick: F)
    where
        F: FnMut(u8),
    {
        let mut current: f64 = min.into();
        let mut x: f64 = 0.0;
        let mut is_going_up = false;

        Self::run(self, || {
            let modo = Self::log(x, 1.0, 1.0, 0.5);

            if !is_going_up && current > min.into() {
                x -= 0.1;
            } else if is_going_up && current < max.into() {
                x += 0.1;
            } else {
                is_going_up = !is_going_up;
            }

            current = (max as f64 * modo).round();
            on_tick(current as u8);
        });
    }

    // TODO: improve the param names
    pub fn log(x: f64, L: f64, k: f64, x0: f64) -> f64 {
        L / (1.0 + E.powf(-1.0 * k * (x - x0)))
    }

    pub fn clean_screen() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}
