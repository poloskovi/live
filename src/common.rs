// Общие объекты - координаты и проч.

// пока отключаю третье измерение. Тестирую на двумерном мире.

extern crate rand;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    //pub z: i32,
}

impl Point{
    
    // Расстояние между точками
    pub fn distantion(p1:Point, p2:Point) ->f32{
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx*dx + dy*dy).sqrt()
    }
    
}

// Направление в сферической системе координат
// углы указаны в градусах
pub struct Direction {
    // азимут
    pub fi: u16,
    // зенит
    // teta: u16,
}

impl Direction {
    pub fn random() -> Direction {
    
        let mut rng = rand::thread_rng();
        Direction {
            fi: rng.gen_range(0, 359),
//             teta: rng.gen_range(0, 359),
        }
    
    }
}

