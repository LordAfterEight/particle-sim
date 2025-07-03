#![allow(static_mut_refs)]

use macroquad::prelude::*;
use macroquad::input::*;
mod world;
use world::*;
pub mod settings;
use crate::settings::values::*;


#[macroquad::main("Rusty Pixels")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut window = macroquad::window::Conf::default();
    window.window_resizable = false;
    let mut world = World::new();

    /*
    world.pixels.push(Pixel::new(world::Element::default(), 400.0, 0.0));
    world.pixels.push(Pixel::new(world::Element::default(), 430.0, 45.0));
    world.pixels.push(Pixel::new(world::Element::default(), 254.0, 20.0));
    world.pixels.push(Pixel::new(world::Element::default(), 400.0, 90.0));
    world.pixels.push(Pixel::new(world::Element::default(), 300.0, 23.0));
    world.pixels.push(Pixel::new(world::Element::default(), 630.0, 73.0));
    world.pixels.push(Pixel::new(world::Element::default(), 754.0, 57.0));
    world.pixels.push(Pixel::new(world::Element::default(), 500.0, 47.0));
    */

    let mut pause_state = false;

    loop {
        //clear_background(BLACK);
        if macroquad::input::is_mouse_button_down(MouseButton::Left) {
            let (pos_x, pos_y) = macroquad::input::mouse_position();
            world.pixels.push(Pixel::new(world::Element::default(), pos_x, pos_y));
        }

        if macroquad::input::is_mouse_button_down(MouseButton::Right) {
            let (pos_x, pos_y) = macroquad::input::mouse_position();
            world.pixels.push(Pixel::new(
                world::Element::new(
                    "Fire".to_string(),
                    macroquad::color::Color::new(1.0,0.3,0.0,1.0),
                    u64::MAX,
                    -0.25,
                    0.0,
                    10.0
                ),
                pos_x,
                pos_y)
            );
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
            next_frame().await
        }
    }
}
