#![allow(static_mut_refs)]

use std::process::exit;

use macroquad::prelude::*;
use macroquad::input::*;
use macroquad::rand::gen_range;
mod world;
use world::*;
pub mod settings;
use crate::settings::values::*;


#[macroquad::main("Rusty Pixels")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut world = World::new();

    let mut pause_state = false;

    loop {
        if macroquad::input::is_key_pressed(KeyCode::Q) {
            exit(1);
        }

        // Get mouse position
        let (pos_x, pos_y) = macroquad::input::mouse_position();

        // Create and draw the pixels
        if pos_x < SCREEN_WIDTH {

            if macroquad::input::is_mouse_button_down(MouseButton::Left) {
                world.pixels.push(Pixel::new(world::Element::default(), pos_x, pos_y));
            }

            if macroquad::input::is_mouse_button_down(MouseButton::Right) {
                let (pos_x, pos_y) = macroquad::input::mouse_position();
                world.pixels.push(Pixel::new(
                    world::Element::new(
                        "Fire".to_string(),
                        macroquad::color::Color::new(gen_range(0.8, 1.0),gen_range(0.0, 0.5),0.0,1.0),
                        100,
                        -0.15,
                        10.0,
                        800.0
                    ),
                    pos_x,
                    pos_y)
                );
            }

            if macroquad::input::is_mouse_button_down(MouseButton::Middle) {
                let (pos_x, pos_y) = macroquad::input::mouse_position();
                world.pixels.push(Pixel::new(
                    world::Element::new(
                        "Metal".to_string(),
                        macroquad::color::Color::new(0.3,0.3,gen_range(0.3,0.4),1.0),
                        u16::MAX,
                        0.0,
                        0.0,
                        22.0
                    ),
                    pos_x,
                    pos_y)
                );
            }
        }

        if macroquad::input::is_key_pressed(KeyCode::Space) {
            match pause_state {
                true  => {
                    pause_state = false;
                    continue
                },
                false => pause_state = true
            }
            std::thread::sleep(std::time::Duration::from_millis(250));
        }

        if !pause_state {
            world.update();
            unsafe { draw_text(&format!("Particles: {}", PIXEL_AMOUNT) as &str, 10.0, 10.0, 16.0, WHITE); }
            draw_text("Press [q] to exit", SCREEN_WIDTH-125.0, 10.0, 16.0, WHITE);
            request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
            next_frame().await
        }
    }
}
