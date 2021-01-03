// Особи

use crate::common as common;
use crate::neuronet as neuronet;

pub enum TypeOfSensor {
    Light,
    Poison
}

// Чувствительность одного сенсора всегда равна 1.
pub struct Sensor {
    typeofsensor: TypeOfSensor,
    // направление сенсора
    direction: common::Direction,
}

// особь
pub struct Osobj {

    // текущие координаты
    position: common::Coordinates,
    
    // направление (от направления особи завивит направление обзора сенсора)
    direction: common::Direction,
    
    // Нейросеть особи. Входной слой: рецепторы, выходной слой: направление движения
    // сила движения зависит от силы органов движения
    brain: neuronet::Neuronet,
    
    // Сенсоры, получают информацию из окружающей среды и передают на вход нейросети
    // индекс сенсора равен индексу входного слоя нейросети
    // Количество ячеек во входном слое определяется количеством сенсоров.
    sensors: Vec<Sensor>,
    
    // накопленная энергия. 
    // Если energy >= massa, тогда происходит деление
    // Если energy <= 0, то особь погибает
    energy: u32,
    
    // Масса, равна сумме клеток мозга
    massa: u32,
    
    // Линейный размер, корень кубический из массы.
    // Влияет на минимальное расстояние между особями
    size: u32,
}

impl Osobj {
    
    pub fn new(position: common::Coordinates, brain: neuronet::Neuronet, sensors: Vec<Sensor>, energy: u32) -> Osobj {
    
        let mut osobj = Osobj{
            position: position,
            brain: brain,
            sensors: sensors,
            energy: energy,
            direction: common::Direction::random(),
            massa: 0,
            size: 0,
        };
        
        osobj.massa = osobj.count_massa();
        osobj.size = osobj.count_size();
        
        osobj
    
    }

    fn count_massa(&self) -> u32 {
        let mut massa = self.brain.count_of_connection() as u32;
        if massa == 0 {
            massa = 1;
        }
        massa
    }
    
    fn count_size(&self) -> u32 {
        // корень кубический из массы
        let mut size = self.massa * 10 / 17;
        if size == 0 {
            size = 1;
        }
        size
    }

}

// простейший мозг: один сенсор, один выход, один скрытый слой
pub fn simple_brain() -> neuronet::Neuronet{
    neuronet::Neuronet::new(vec![1,1,1])
}

pub fn simple_sensors() -> Vec<Sensor> {
    let sensor = Sensor{
        typeofsensor: TypeOfSensor::Light,
        direction: common::Direction::random(),
    };
    vec![sensor,]
}
