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

extern crate neuronet;

use crate::common::{Point, Direct, Force};
use crate::neuronet::{Tdata, FORMFACTOR, NeuroMatrix, Neuronet, Sigmoida};

use crate::memory::{Memory, MemoryCell};

extern crate rand;
use rand::Rng;

pub enum TypeOfSensor {
    Light,
    #[allow(dead_code)]
    Poison
}

// Сенсор. Чувствительность одного сенсора всегда равна 1.
pub struct Sensor {
    #[allow(dead_code)]
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

// особь
pub struct Osobj {

    // текущие координаты
    pub position: Point,
    
    // Нейросеть рецепторы-органы движения
    pub nnodes: Vec<usize>,
    brain: Neuronet,
    
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
    pub massa: f32,
    
    // Память состояний
    memory: Memory, //временно pub
    memory_capacity: usize,
    
    // окрестность состояний входов нейросети, в которой происходит поиск оптимального выхода
    len_memorycell_min: i32,
    
    // расстояние от ближайшей ячейки памяти, начиная с которого нужно делать новую ячейку памяти
    len_memorycell_max: i32,

    // Требуется пересчет нейросети
    pub need_train_brain: bool,

    // накопление энергии на предыдущем шаге
    pub prev_gain_energy: Option<f32>,

    // особь мертва
    pub dead: bool,
    
}

// Результат поиска в памяти
pub enum ResultFindInMemory{
    TryModifyCell(usize),//попробовать модифицировать выходной сигнал этой ячейки
    MakeNewCell,// Нужно создавать новую ячейку
    MoveByNeuronet// нужно двигаться по расчету нейросети
}

#[allow(dead_code)]
impl Osobj {
    
    pub fn new(
        brain_configuration: Vec<usize>,
        memory_capacity: usize,
        sensors: Vec<Sensor>, 
        legs: Vec<Leg>, 
        energy: f32
        ) -> Osobj {
    
        //добавить входной и выходной слои нейросети
        let mut nnodes = Vec::with_capacity(brain_configuration.len()+2);
        nnodes.push(legs.len());
        for node in &brain_configuration{
            nnodes.push(*node);
        }
        nnodes.push(sensors.len());

        let mut osobj = Osobj{
            position: Point{x:0.0, y:0.0},
            brain: Neuronet::new(&nnodes),
            nnodes: nnodes,
            memory: Memory::new(memory_capacity),
            memory_capacity: memory_capacity,
            sensors: sensors,
            legs: legs,
            energy: energy,
            massa: 0.0,
            len_memorycell_min: 800,
            len_memorycell_max: 3200,
            need_train_brain: false,
            prev_gain_energy: None,
            dead: false,
        };
        
        osobj.massa = osobj.count_massa();
        
        osobj
    
    }

    pub fn copy_modify(&self) -> Osobj{

        let mut brain_configuration = Vec::with_capacity(self.nnodes.len()-2);
        for i in 1..&self.nnodes.len()-1{
            brain_configuration.push(self.nnodes[i]);
        }
        //слегка модифицируем конфигурацию нейросети
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, brain_configuration.len());//случайный слой
        let mut low = brain_configuration[i] - 1;
        if low < 1{
            low = 1;
        }
        let high = brain_configuration[i] + 1;
        brain_configuration[i] = rng.gen_range(low, high+1);

        let mut new_osobj = Osobj::new(
            brain_configuration,
            self.memory_capacity,
            simple_sensors(),
            simple_legs(),
            self.energy/2.0
        );
        new_osobj.position = self.position;
        new_osobj
    }

    // добавить "массу" памяти
    fn count_massa(&self) -> f32 {
        let mut massa = self.brain.count_of_connection() as f32;
        if massa == 0.0 {
            massa = 1.0;
        }
        massa
    }
    
    #[allow(dead_code)]
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
    pub fn get_brain_output(&self, input: &NeuroMatrix, sigmoida: &Sigmoida) -> NeuroMatrix {
        self.brain.getoutput(input, sigmoida)
    }
    
    // сумма вектора усилий ног в результате команды нейросети
    pub fn common_force(&self, brain_output: &NeuroMatrix) -> Force {
    
        // нужно сложить векторы усилий ног
        let mut forces = Vec::<Force>::new();
        for (i, value) in self.legs.iter().enumerate() {
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
        let koeff = 20.0; // коэффициент, подобрать экспериментально
        let r = force.f * koeff / self.massa;
        self.position.movement(r, force.direct);
        
        // ограничение "аквариума"
        let r_max = 400.0;// радиус аквариума
        let mut polar = self.position.to_polar();
        if polar.r > r_max {
            polar.r = r_max;
            self.position = polar.to_decart();
        }

    }
    
    pub fn change_energy(&mut self, sol_force: f32, poison_force: f32){
        let expenses = 0.1;  // постоянные расходы на перемещение
                            // можно сделать зависимыми от усилий ног
        self.energy = self.energy
            + (sol_force-poison_force)/1000.0
            - expenses
    }
    
    // поиск, есть ли в памяти похожая ситуация
    pub fn find_in_memory(&self, input: &NeuroMatrix) -> ResultFindInMemory{
        let in_memory = self.memory.find_near(input);
        match in_memory {
            Some((index, distance)) => {
                if distance <= self.len_memorycell_min {
                    ResultFindInMemory::TryModifyCell(index)
                } else if distance > self.len_memorycell_max {
                    ResultFindInMemory::MakeNewCell
                } else {
                    ResultFindInMemory::MoveByNeuronet
                }
            },
        None=> ResultFindInMemory::MakeNewCell
        }
        
    }
    
    pub fn add_to_memory(&mut self,
        input: &NeuroMatrix,
        output: &NeuroMatrix,
        delta_gain_energy: Option<f32>,
        tact: u32)
    {
        self.memory.add(input, output, delta_gain_energy, tact);
        self.need_train_brain = true;
    }
    
    pub fn replace_in_memory(
        &mut self,
        index:usize,
        input: &NeuroMatrix,
        output: &NeuroMatrix,
        delta_gain_energy: Option<f32>,
        tact: u32
    ){
        self.memory.replace(index, input, output, delta_gain_energy, tact);
        self.need_train_brain = true;
    }
    
    pub fn get_memory_cell(
        &self,
        index: usize,
        tact: u32
    ) -> &MemoryCell{
        self.memory.get(index, tact)
    }
    
    pub fn brain_training(&mut self, sigmoida: &Sigmoida){
        // "забываем" все
        self.brain = Neuronet::new(&self.nnodes);
        for memorycell in self.memory.cells.iter(){
            for _ in 0..5{
                self.brain.training(&memorycell.input, &memorycell.output, sigmoida);
            }
        }
    }

}

// Особь для тестирования
pub fn sample_osobj() -> Osobj{

    let brain_configuration = vec![10, 10, 10];
    let memory_capacity = 10;
    let start_energy = 10.0;
    
    Osobj::new(
        brain_configuration,
        memory_capacity,
        simple_sensors(),
        simple_legs(),
        start_energy
    )

}

pub fn simple_legs() -> Vec<Leg>{

    let mut result = Vec::with_capacity(4);

    result.push(Leg{direct: Direct{fi:0.0}});
    result.push(Leg{direct: Direct{fi:90.0}});
    result.push(Leg{direct: Direct{fi:180.0}});
    result.push(Leg{direct: Direct{fi:270.0}});

    result
}

pub fn simple_sensors() -> Vec<Sensor>{

    let mut result = Vec::with_capacity(4);

    result.push(Sensor {
        typeofsensor: TypeOfSensor::Light,
        direct: Direct{fi: 0.0}
    });
    result.push(Sensor {
        typeofsensor: TypeOfSensor::Light,
        direct: Direct{fi: 90.0}
    });
    result.push(Sensor {
        typeofsensor: TypeOfSensor::Light,
        direct: Direct{fi: 180.0}
    });
    result.push(Sensor {
        typeofsensor: TypeOfSensor::Light,
        direct: Direct{fi: 270.0}
    });

    result

}
