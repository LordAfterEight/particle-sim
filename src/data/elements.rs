use crate::world::*;
use macroquad::rand::gen_range;

pub struct Elements {
    pub smoke: Element,
    pub fire: Element,
    pub sand: Element,
    pub metal: Element,
}

impl Elements {
    pub fn init() -> Elements {
        let smoke = Element::new(
            "Smoke".to_string(),
            macroquad::color::Color::new(0.2,0.2,0.2,1.0),
            20,
            -0.01,
            5.0,
            80.0,
        );

        let fire = Element::new(
            "Fire".to_string(),
            macroquad::color::Color::new(gen_range(0.8, 1.0),gen_range(0.0, 0.5),0.0,1.0),
            40,
            -0.15,
            10.0,
            800.0,
        );

        let metal = Element::new(
            "Metal".to_string(),
            macroquad::color::Color::new(0.3,0.3,gen_range(0.3,0.4),1.0),
            u16::MAX,
            0.0,
            0.0,
            22.0,
        );

        let sand = Element::default();


        let elements = Elements {
            smoke,
            fire,
            metal,
            sand,
        };
        return elements
    }
}
