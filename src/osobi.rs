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

use crate::common::{Point, Direct, Force};
use crate::neuronet::{Tdata, FORMFACTOR, Matrix, Neuronet, Sigmoida};

pub enum TypeOfSensor {
    Light,
    Poison
}

// Сенсор. Чувствительность одного сенсора всегда равна 1.
pub struct Sensor {
    typeofsensor: TypeOfSensor,
    // направление сенсора
    direct: Direct,
}

impl Sensor {

    pub fn signal_on_sensor(&self, force: &Force) -> f32 {
        // угол между осью сенсора и источником сигнала
        let delta_angle = - self.direct.delta(force.direct.revers());
        
        // сенсор видит сигнал, если направлен в ту же полусферу, откуда приходит сигнал
        let signal = (delta_angle * std::f32::consts::PI / 180.0).cos() * force.f;
        if signal < 0.0 {
            0.0
        }else{
            signal
        }
    }
}

// Орган движения. Сила всегда равна 1.
pub struct Leg {
    direct: Direct,
}

// Ячейка памяти особи
pub struct MemoryCell {
    pub input: Matrix,
    pub output: Matrix,
    // изменение накопления энергии в результате этой реакции input->output
    // по сравнению с накоплением энергии на предыдущем шаге
    pub izm_delta_energy: f32,
    // проведена тренировка нейросети на этом наборе
    pub trained: bool,
}

pub struct Memory {
    pub cells: Vec<MemoryCell>,
}

impl Matrix {
    // "расстояние" между векторами
    // разница между наборами входных сигналов
    fn distance(&self, other:&Matrix) -> i32{
    
        Matrix::panic_if_not_same_size(self, other);

        let mut result = 0;
        for row in 0..self.nrow {
            for col in 0..self.ncol {
                let d = self.get(row,col) - other.get(row,col);
                result = result + d*d;
            }
        }
        result
    }
}

impl Memory{

    fn new() -> Memory {
        Memory{
            cells: Vec::<MemoryCell>::new(),
        }
    }
    
    fn get(&self, index: usize) -> &MemoryCell{
        &self.cells[index]
    }
    
    fn add(&mut self, input: Matrix, output: Matrix, izm_delta_energy: f32){
        let memorycell = MemoryCell{
            input: input,
            output: output,
            izm_delta_energy: izm_delta_energy,
            trained: false,
        };
        self.cells.push(memorycell);
    }
    
    fn replace(&mut self, index: usize, input: Matrix, output: Matrix, izm_delta_energy: f32){
        let memorycell = MemoryCell{
            input: input,
            output: output,
            izm_delta_energy: izm_delta_energy,
            trained: false,
        };
        self.cells[index] = memorycell;
    }
    
    // найти ближайший образец.
    // возвращает индекс ячейки памяти
    fn find_near(&self, input: &Matrix) -> Option<(usize, i32)>{
//     Option<&MemoryCell>{

//         рабочий код для возврата индекса ближайшего вектора
//         self.cells
//             .iter()
//             .enumerate()
//             .min_by_key(|(_idx, p)| p.input.distanсе(input))
//             .map(|(idx, _val)| idx)
            
        // Мне надо получить индекс ближайшего вектора и величину дистанции.
        self.cells
            .iter()
            .map(|p| p.input.distance(input))
            .enumerate()
            .min_by_key(|(_idx, p)| *p)
            
    }
    
}

pub fn test_memory_find_near(){
    
    let mut memory = Memory::new();
    
    let input = Matrix::new_rand(1, 4, 0, 10, false);
    println!(" 0: {}", &input);
    let output = Matrix::new(1, 4);
    memory.add(input, output, 0.0);
//     
    let input = Matrix::new_rand(1, 4, 0, 255, false);
    println!(" n: {}", &input);
    
    let result = memory.find_near(&input);
//     println!("ближайшее: {}", index);
    match result {
        Some(x) => println!("ближайшее: индекс {}, расстояние {}", x.0, x.1),//&x.input),
        None    => println!("ближайшего элемента нет..."),
    }    
    
}

// особь
pub struct Osobj {

    // текущие координаты
    pub position: Point,
    
    // направление (от направления особи завивит направление обзора сенсора)
    //      сделать потом
    // direct: common::Direct,
    
    // Нейросеть рецепторы-органы движения
    brain: Neuronet,
    
    // Единая нейросеть: 
    // Вход (сигнал на рецепторах) - анализатор - органы движения - окружающая среда - изменение целевой функции 
    // (в простейшем случае целевая функция - это накопление энергии)
    //      Нет, так нельзя. Нейросеть окружающей среды должна иметь на входе и движение ног, 
    //      и значения параметров окружающей среды, полученные через сенсоры.
    //      Кроме того, непонятно, что есть целевая функция. Стремиться к какой-то цифре? 
    //      Но вдалеке от источника света ее не достичь
    //      Вариант с памятью лучше, так как на примерно одинаковых входных значениях 
    //      выбирается движение, дающее максимальный прирост 
    //      целевой функции для данных условий
    //      
    // brain_and_env: Neuronet,
    
    // позиция органов движения в нейросети
    // pos_legs: usize,
    
    // Сенсоры, получают информацию из окружающей среды и передают на вход нейросети
    // индекс сенсора равен индексу входного слоя нейросети
    // Количество ячеек во входном слое определяется количеством сенсоров.
    pub sensors: Vec<Sensor>,
    
    // "Ноги", дают толчок к перемещению. Фактическое перемещение равно сумме векторов ног, деленное на массу особи.
    legs: Vec<Leg>,
    
    // накопленная энергия. 
    // Если energy >= massa, тогда происходит деление
    // Если energy <= 0, то особь погибает
    pub energy: f32,
    
    // Масса, равна сумме клеток мозга
    massa: f32,
    
    // Линейный размер, корень кубический из массы.
    // Влияет на минимальное расстояние между особями
    //      сделать потом
    // size: u32,
    
    // Память состояний
    memory: Memory,
    max_memory_cells: usize,
    
    // Попробуем вместо памяти состояний использовать расчет "нейросети" окружаюющей среды
    //environment: Neuronet,
}

impl Osobj {
    
    pub fn new(
        position: Point, 
        brain: Neuronet, 
        memory: Memory,
        max_memory_cells: usize,
        sensors: Vec<Sensor>, 
        legs: Vec<Leg>, 
        energy: f32
        ) -> Osobj {
    
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
            max_memory_cells: max_memory_cells,
            memory: memory,
            sensors: sensors,
            legs: legs,
            energy: energy,
            massa: 0.0,
        };
        
        osobj.massa = osobj.count_massa();
//         osobj.size = osobj.count_size();
        
        osobj
    
    }

    // добавить "массу" памяти
    fn count_massa(&self) -> f32 {
        let mut massa = self.brain.count_of_connection() as f32;
        if massa == 0.0 {
            massa = 1.0;
        }
        massa
    }
    
    fn count_size(&self) -> u32 {
        // корень кубический из массы
        todo!("сделать вычисление корня кубического")
    }
    
    pub fn signal_on_sensors(&self, forces: Vec<Force>) -> Vec<Tdata> {
        let mut result = Vec::<Tdata>::new();
        for sensor in self.sensors.iter() {
            let mut signal = 0.0;
            for force in forces.iter() {
                signal = signal + sensor.signal_on_sensor(force);
            }
            result.push(signal as Tdata);
        }
        result
    }
    
    // выходной сигнал нейросети
    pub fn get_brain_output(&self, input: &Matrix, sigmoida: &Sigmoida) -> Matrix {
        self.brain.getoutput(input, sigmoida)
    }
    
    // сумма вектора усилий ног в результате команды нейросети
    pub fn common_force(&mut self, brain_output: &Matrix) -> Force {
    
        // нужно сложить векторы усилий ног
        let mut forces = Vec::<Force>::new();
        for (i, value) in self.legs.iter().enumerate() {
//             println!("{} {} {}", i, brain_output.get(0, i), value.direct);
            forces.push(
                Force{
                    f: 1.0 * // сила каждой ноги = 1
                        brain_output.get(0, i) as f32 / (FORMFACTOR as f32)
                        // максимальный сигнал на выходе нейросети дает усилие = 1
                    ,
                    direct: value.direct,
                }
            );
            
        }
        Force::common_force(&forces)

    }
    
    pub fn movement(&mut self, force: Force) {
        
        // длина перемещения
        let koeff = 100.0; // коэффициент, подобрать экспериментально
        let r = force.f * koeff / self.massa;
        self.position.movement(r, force.direct);
        
        // ограничение "аквариума"
        // переделать на функцию "мира" (сделать объект "мир")
        if self.position.y > -50.0 {
            self.position.y = -50.0;
        }
    }
    
    pub fn change_energy(&mut self, sol_force: f32){
        self.energy = self.energy + sol_force;
    }
    
    pub fn find_in_memory(&self, input: &Matrix) -> Option<(usize, i32)>{
        self.memory.find_near(input)
    }
    
    pub fn put_to_memory(&mut self, input: Matrix, output: Matrix, delta_energy: f32){
        self.memory.add(input, output, delta_energy);
    }
    
    pub fn replace_in_memory(&mut self, index:usize, input: Matrix, output: Matrix, delta_energy: f32){
        self.memory.replace(index, input, output, delta_energy);
    }
    
    pub fn get_memory_cell(&self, index: usize) -> &MemoryCell{
        self.memory.get(index)
    }
    
    pub fn brain_training(&mut self, input: &Matrix, output: &Matrix, sigmoida: &Sigmoida){
        self.brain.training(input, output, sigmoida);
    }

}

// простейший мозг: один сенсор, один выход, один скрытый слой
pub fn simple_brain() -> Neuronet{
    Neuronet::new(vec![1,1,1])
}

pub fn simple_sensors() -> Vec<Sensor> {
    let sensor = Sensor{
        typeofsensor: TypeOfSensor::Light,
        direct: Direct::random(),
    };
    vec![sensor,]
}

pub fn simple_legs() -> Vec<Leg> {
    let leg = Leg{
        direct: Direct::random(),
    };
    vec![leg,]
}

// Особь для тестирования
pub fn sample_osobj() -> Osobj{

    let count_of_leg = 4;
    let count_of_sensors = 4;

    let brain = Neuronet::new(vec![count_of_sensors, 10, 10, count_of_leg]);
//     let environment = Neuronet::new(vec![count_of_leg, 10, 10, count_of_sensors]);
    let memory = Memory::new();
    
    let leg_1 = Leg{
        direct: Direct{fi:0.0}
    };
    let leg_2 = Leg{
        direct: Direct{fi: 90.0}
    };
    let leg_3 = Leg{
        direct: Direct{fi: 180.0}
    };
    let leg_4 = Leg{
        direct: Direct{fi: 270.0}
    };
    let legs = vec![leg_1, leg_2, leg_3, leg_4];
    
    let sensor_1 = Sensor {
        typeofsensor: TypeOfSensor::Light,
        direct: Direct{fi: 0.0}
    };
    let sensor_2 = Sensor {
        typeofsensor: TypeOfSensor::Light,
        direct: Direct{fi: 90.0}
    };
    let sensor_3 = Sensor {
        typeofsensor: TypeOfSensor::Light,
        direct: Direct{fi: 180.0}
    };
    let sensor_4 = Sensor {
        typeofsensor: TypeOfSensor::Light,
        direct: Direct{fi: 270.0}
    };
    let sensors = vec![sensor_1, sensor_2, sensor_3, sensor_4];
    
    let position = Point{
        x: 100.0,
        y: -100.0,
    };
    
    let start_energy = 100.0;
    let max_memory_cells = 10;
    
    Osobj::new(
        position, 
        brain,
        memory,
        max_memory_cells,
        sensors,
        legs,
        start_energy
    )

}
