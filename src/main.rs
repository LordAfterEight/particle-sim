use macroquad::prelude::*;
mod world;
use world::*;
pub mod settings;
use crate::settings::values::*;


#[macroquad::main("MyGame")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut window = macroquad::window::Conf::default();
    window.window_resizable = false;
    let mut world = World::new();

    world.pixels.push(Pixel::new(world::Element::default(), 400.0, 0.0));
    world.pixels.push(Pixel::new(world::Element::default(), 430.0, 45.0));
    world.pixels.push(Pixel::new(world::Element::default(), 254.0, 20.0));
    world.pixels.push(Pixel::new(world::Element::default(), 400.0, 90.0));
    world.pixels.push(Pixel::new(world::Element::default(), 300.0, 23.0));
    world.pixels.push(Pixel::new(world::Element::default(), 630.0, 73.0));
    world.pixels.push(Pixel::new(world::Element::default(), 754.0, 57.0));
    world.pixels.push(Pixel::new(world::Element::default(), 500.0, 47.0));

    world.pixels.push(Pixel::new(
        world::Element::new(
            "Fire".to_string(),
            macroquad::color::Color::new(1.0,0.6,0.0,1.0),
            u64::MAX,
            -0.25
        ),
        100.0,
        200.0)
    );

    loop {
        //clear_background(BLACK);
        world.update();
        next_frame().await
    }
}
