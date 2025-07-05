use macroquad::rand::gen_range;

use crate::settings::values::*;


pub static mut FRAME: [[bool;SCREEN_HEIGHT as usize + 30];SCREEN_WIDTH as usize + 30] = [[true;SCREEN_HEIGHT as usize + 30]; SCREEN_WIDTH as usize + 30];

#[derive(Clone)]
pub struct Element {
    pub name: String,
    pub color: macroquad::color::Color,
    pub lifetime: u16,
    pub weight: f32,
    pub max_velocity: f32,
    pub temperature: f32,
}

impl Default for Element {
    /// Returns the default Element: Sand
    fn default() -> Self {
        Self {
            name: "Sand".to_string(),
            color: macroquad::color::Color::new(1.0,0.9,0.5,1.0),
            lifetime: u16::MAX,
            weight: 1.0,
            max_velocity: 10.0,
            temperature: 22.0,
        }
    }
}

impl Element {

    pub fn new(
        name: String,
        color: macroquad::color::Color,
        lifetime: u16,
        weight: f32,
        max_velocity: f32,
        temperature: f32,
    ) -> Self {
        Self {
            name,
            color,
            lifetime,
            weight,
            max_velocity,
            temperature,
        }
    }

    pub fn empty() -> Self {
        Self {
            name: "None".to_string(),
            color: macroquad::color::Color::new(0.0,0.0,0.0,0.0),
            lifetime: u16::MAX,
            weight: 0.0,
            max_velocity: 0.0,
            temperature: 0.0,
        }
    }
}

pub struct Pixel {
    pub x: f32,
    pub x_velocity: f32,
    pub y: f32,
    pub y_velocity: f32,
    pub element: Element,
    pub to_be_removed: bool
}

impl Pixel {
    pub fn new(element: &Element, pos_x: f32, pos_y: f32) -> Self {
        unsafe {
            if FRAME[pos_x as usize][pos_y as usize] {
                PIXEL_AMOUNT += 1;
                Self {
                    x: pos_x,
                    x_velocity: gen_range(-1.0,1.0),
                    y: pos_y,
                    y_velocity: 0.0,
                    element: element.clone(),
                    to_be_removed: false
                }
            } else {
                Self {
                    x: -10.0,
                    x_velocity: 0.0,
                    y: -10.0,
                    y_velocity: 0.0,
                    element: Element::empty(),
                    to_be_removed: true
                }
            }
        }
    }

    pub fn update(&mut self) {
        unsafe {
            let mut elements = crate::elements::Elements::init();
            FRAME[self.x as usize][self.y as usize] = true;

            elements.smoke.color = macroquad::color::Color::new(gen_range(0.1,0.2),gen_range(0.1,0.2),gen_range(0.1,0.2),1.0);
            elements.smoke.lifetime = gen_range(10, 30);

            if FRAME[self.x as usize][(self.y + SCALING) as usize] {
                macroquad::prelude::draw_rectangle(self.x, self.y, SCALING, SCALING, self.element.color);

                if (self.element.lifetime < u16::MAX) && self.element.lifetime > 0 {
                    self.element.lifetime -= 1;
                }


                self.y_velocity += self.element.weight;

                println!("{} at X: {:.0} | Y: {:.0} | lifetime: {} | ID: {}",
                    self.element.name,
                    self.x,
                    self.y,
                    self.element.lifetime,
                    PIXEL_AMOUNT
                );

                self.x += self.x_velocity * SCALING;

                self.x_velocity *= 0.9;

                if self.y < SCREEN_HEIGHT - SCALING {

                    if (self.y + self.y_velocity * SCALING) >= SCREEN_HEIGHT {
                        self.y = SCREEN_HEIGHT - SCALING;
                        self.x_velocity = 0.0;
                    } else { self.y += (self.y_velocity * SCALING) * (gen_range(0.8, 1.2)); }

                } else { self.y = SCREEN_HEIGHT - SCALING }

                if self.element.lifetime == 15 {
                    self.element = elements.smoke;
                }

                if self.y < 0.0 || self.element.lifetime < 0 {
                    self.to_be_removed = true;
                }
            }
            FRAME[self.x as usize][self.y as usize] = false;
        }
    }
}

pub struct World {
    pub pixels: Vec<Pixel>
}

impl World {
    pub fn new() -> Self {
        Self {
            pixels: Vec::new()
        }
    }
    pub fn update(&mut self) {
        for pixel in &mut self.pixels {
            pixel.update();
        }
        self.pixels.retain(|pixel| !pixel.to_be_removed);
        unsafe { PIXEL_AMOUNT = self.pixels.len(); }
    }
}

