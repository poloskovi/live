// Общие объекты - координаты и проч.

// пока отключаю третье измерение. Тестирую на двумерном мире.

extern crate rand;
use rand::Rng;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    //pub z: i32,
}

#[allow(dead_code)]
impl Point{
    // Расстояние между точками
    pub fn distance(p1:Point, p2:Point) ->f32{
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx*dx + dy*dy).sqrt()
    }
}

#[allow(dead_code)]
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x={}, y={}", self.x, self.y)
    }
}

// Направление в сферической системе координат
// углы указаны в градусах
#[derive(Copy, Clone)]
pub struct Direct{
    // азимут
    pub fi: f32,
    // зенит
    // teta: u16,
}

#[allow(dead_code)]
impl Direct{
    pub fn random() -> Direct{
    
        let mut rng = rand::thread_rng();
        Direct{
//             fi: rng.gen_range(0, 359),
            fi: rng.gen::<f32>()*360.0,
        }
    
    }
    
    //противоположное направление
    pub fn revers(&self) -> Direct{
        let mut fi = self.fi-180.0;
        if fi < 0.0{
            fi = fi + 360.0;
        }
        Direct {
            fi: fi,
        }
    }
    
    // угол между направлениями
    pub fn delta(&self, other: Direct) -> f32{
        self.fi - other.fi
    }
}

#[allow(dead_code)]
impl fmt::Display for Direct{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fi={} град.", self.fi)
    }
}

#[derive(Copy, Clone)]
pub struct Force{
    pub f: f32,
    pub direct: Direct,
    //pub z: i32,
}

#[allow(dead_code)]
impl fmt::Display for Force {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "сила={}, направление=({})", self.f, self.direct)
    }
}


