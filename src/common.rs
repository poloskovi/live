// Общие объекты - координаты и проч.

extern crate rand;
use rand::Rng;

pub struct Coordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// Направление в сферической системе координат
pub struct Direction {
    // азимут
    fi: u16,
    // зенит
    teta: u16,
}

impl Direction {
    pub fn random() -> Direction {
    
        let mut rng = rand::thread_rng();
        Direction {
            fi: rng.gen_range(0, 359),
            teta: rng.gen_range(0, 359),
        }
    
    }
}

