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
    // перевод в декартову систему координат
    pub fn polar_to_decart(r: f32, direct: Direct) -> Point {
        let angle_radian = direct.fi * std::f32::consts::PI / 180.0;
        Point{
            x: angle_radian.cos() * r,
            y: angle_radian.sin() * r,
        }
    }
    // перевод в полярную систему координат
    pub fn to_polar(&self) -> Polar {
    
        let r;
        let mut fi;
        let r_sqr = self.x*self.x + self.y*self.y;

        if r_sqr == 0.0{
            r = 0.0;
            fi = 0.0;
        }else{
            r = r_sqr.sqrt();
            fi = (self.y / r).asin()
                * 180.0/std::f32::consts::PI; // перевод в градусы
                        
            if self.y < 0.0 && self.x < 0.0 {
                fi = -180.0 - fi
            }else if self.y > 0.0 && self.x < 0.0 {
                fi = 180.0 - fi
            }
        
        }
        Polar::new(r, Direct::new(fi))
    }
    pub fn movement(&mut self, r: f32, direct: Direct){
        let delta = Point::polar_to_decart(r, direct);
        self.x = self.x + delta.x;
        self.y = self.y + delta.y;
    }
}

// координаты в полярной системе координат
#[derive(Copy, Clone)]
pub struct Polar{
    pub r: f32,
    pub direct: Direct
}

impl Polar{
    pub fn new(r:f32, direct: Direct) -> Polar{
        Polar{
            r,
            direct,
        }
    }
    pub fn to_decart(self) -> Point {
        let angle_radian = self.direct.fi * std::f32::consts::PI / 180.0;
        Point{
            x: self.r * angle_radian.cos(),
            y: self.r * angle_radian.sin(),
        }
    }
}

#[allow(dead_code)]
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x={:5.2}, y={:5.2}", self.x, self.y)
    }
}

#[allow(dead_code)]
impl fmt::Display for Polar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r={:6.0}, fi={:5.0}", self.r, self.direct.fi)
    }
}

// Направление в сферической системе координат
// углы указаны в градусах
#[derive(Copy, Clone, Debug)]
pub struct Direct{
    // азимут
    pub fi: f32,
    // зенит
    // teta: u16,
}

#[allow(dead_code)]
impl Direct{

    pub fn new(fi:f32) -> Direct{
        Direct{
            fi
        }
    }

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

#[derive(Copy, Clone, Debug)]
pub struct Force{
    pub f: f32,
    pub direct: Direct,
    //pub z: i32,
}

impl Force{
    // результирующая сила (сумма векторов сил)
    pub fn common_force(forces: &Vec::<Force>) -> Force {
        let mut fx = 0.0;
        let mut fy = 0.0;
        for force in forces.iter() {
            let p = Point::polar_to_decart(force.f, force.direct);
            fx = fx + p.x;
            fy = fy + p.y;
        }
        let polar = Point{
            x: fx,
            y: fy,
        }.to_polar();
        
        Force{
            f: polar.r,
            direct: polar.direct,
        }
    }
}

#[allow(dead_code)]
impl fmt::Display for Force {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "сила={}, направление=({})", self.f, self.direct)
    }
}

