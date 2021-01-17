// Особи

// Предлагаю такую схему обучения:
//  Особь имеет память из N элементов. Каждой элемент памяти содержит:
//      - вектор входящих сигналов [U_вх]
//      - вектор исходящих сигналов (реакций органов движения)
//      - изменение энергии в результате этой реакции.
//  Для текущего вектора входящих сигналов в памяти ищется элемент с наиболее близким [U_вх].
//  Если он достаточно близок к текущему, то:
//      1. Вычисляется [U_вых]. 
//      2. На [U_вых] накладывается некоторое изменение Дельта: [U_вых] + [Дельта] = [U вых']
//      3. Особь совершает движение.
//      4. Определяется изменение энергии в результате этого движения.
//          Если оно лучше образца, то сеть тренируется по [U вых']. Образец в памяти заменяется на новый.
//          Если оно хуже образца, образцу добавляется вес.
// Или так:

use crate::common::{Coordinates, Direction};
use crate::neuronet::{Matrix, Neuronet};

pub enum TypeOfSensor {
    Light,
    Poison
}

// Сенсор. Чувствительность одного сенсора всегда равна 1.
pub struct Sensor {
    typeofsensor: TypeOfSensor,
    // направление сенсора
    direction: Direction,
}

// Орган движения. Сила всегда равна 1.
pub struct Leg {
    direction: Direction,
}

// Ячейка памяти особи
struct MemoryCell {
    input: Matrix,
    output: Matrix,
    // изменение энергии на этом образце
    delta_energy: i16,
    // количество истинных срабатываний, когда образец из ячейки работал лучше измененного
    weight: u32,
}

struct Memory {
    cells: Vec<MemoryCell>,
}

impl Matrix {
    fn dist(&self, other:&Matrix) -> u32{
        0
    }
}

impl Memory{
    fn new() -> Memory {
        Memory{
            cells: Vec::<MemoryCell>::new(),
        }
    }
    // найти ближайший образец.
    // возвращает индекс ячейки памяти
    fn find_nearest(&self, input: &Matrix) -> Option<&MemoryCell>{
        self.cells.iter().min_by_key(|p| p.input.dist(input))
        // todo!("")
    }
}

// особь
pub struct Osobj {

    // текущие координаты
    position: Coordinates,
    
    // направление (от направления особи завивит направление обзора сенсора)
    //      сделать потом
    // direction: common::Direction,
    
    // Нейросеть рецепторы-органы движения
    brain: Neuronet,
    
    // Сенсоры, получают информацию из окружающей среды и передают на вход нейросети
    // индекс сенсора равен индексу входного слоя нейросети
    // Количество ячеек во входном слое определяется количеством сенсоров.
    sensors: Vec<Sensor>,
    
    // "Ноги", дают толчок к перемещению. Фактическое перемещение равно сумме векторов ног, деленное на массу особи.
    legs: Vec<Leg>,
    
    // накопленная энергия. 
    // Если energy >= massa, тогда происходит деление
    // Если energy <= 0, то особь погибает
    energy: u32,
    
    // Масса, равна сумме клеток мозга
    massa: u32,
    
    // Линейный размер, корень кубический из массы.
    // Влияет на минимальное расстояние между особями
    //      сделать потом
    //size: u32,
    
    // Память состояний
    memory: Memory
}

impl Osobj {
    
    pub fn new(position: Coordinates, 
        brain: Neuronet, 
        sensors: Vec<Sensor>, 
        legs: Vec<Leg>, 
        energy: u32) -> Osobj {
    
        // количество органов движения равно количеству выходов нейросети
        let n_legs = brain.n_outputs();
        let n_sensors = brain.n_inputs();
        
        if sensors.len() != n_sensors {
            panic!("Количество сенсоров особи не равно количеству входов нейросети");
        };
    
        if legs.len() != n_legs{
            panic!("Количество ног особи не равно количеству выходов нейросети");
        };
    
        let mut osobj = Osobj{
            position: position,
            brain: brain,
            sensors: sensors,
            legs: legs,
            energy: energy,
            memory: Memory::new(),
            // direction: common::Direction::random(),
            massa: 0,
        };
        
        osobj.massa = osobj.count_massa();
//         osobj.size = osobj.count_size();
        
        osobj
    
    }

    // добавить "массу" памяти
    fn count_massa(&self) -> u32 {
        let mut massa = self.brain.count_of_connection() as u32;
        if massa == 0 {
            massa = 1;
        }
        massa
    }
    
    fn count_size(&self) -> u32 {
        // корень кубический из массы
        todo!("сделать вычисление корня кубического")
    }

}

// простейший мозг: один сенсор, один выход, один скрытый слой
pub fn simple_brain() -> Neuronet{
    Neuronet::new(vec![1,1,1])
}

pub fn simple_sensors() -> Vec<Sensor> {
    let sensor = Sensor{
        typeofsensor: TypeOfSensor::Light,
        direction: Direction::random(),
    };
    vec![sensor,]
}

pub fn simple_legs() -> Vec<Leg> {
    let leg = Leg{
        direction: Direction::random(),
    };
    vec![leg,]
}
