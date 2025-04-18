use i_am_not_away::{Animator, AnimatorConfig};
use mouse_rs::Mouse;

fn main() {
    println!("Press [Ctrl]+[C] to quit...");

    let animator = Animator::new(Some(AnimatorConfig {
        target_fps: 24,
        speed: 100,
    }));

    animator.oscillate(1, 100, 100, |x| {
        println!("{}", "▓".repeat(x.into()));
    });
}

fn mouse_movement() {
    let mouse = Mouse::new();

    let animator = Animator::new(Some(AnimatorConfig {
        target_fps: 60,
        speed: 700,
    }));

    animator.oscillate(0, 100, 100, |x| {
        mouse
            .move_to(100, 500 + x as i32)
            .expect("Could not move the mouse");
    });
}
