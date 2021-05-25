extern crate neuronet;
use neuronet::NeuroMatrix;
use crate::neuroadd::MatrixAdditions2;

// Ячейка памяти особи
pub struct MemoryCell {
    pub input: NeuroMatrix,
    pub output: NeuroMatrix,
    // изменение накопления энергии в результате этой реакции input->output
    // по сравнению с накоплением энергии на предыдущем шаге
    pub delta_gain_energy: Option<f32>,
    // проведена тренировка нейросети на этом наборе
    //pub n_trained: i16, // убрать?
    pub last_used: u32, // такт, на котором использовано последний раз
}

pub struct Memory {
    pub cells: Vec<MemoryCell>,
    capacity: usize,
    // перенести сюда len_memorycell_min и max ?
}

impl Memory{

    pub fn new(capacity: usize) -> Memory {
        Memory{
            cells: Vec::<MemoryCell>::with_capacity(capacity),
            capacity
        }
    }
    
    pub fn get(&self, index: usize, _tact:u32) -> &MemoryCell{
        let cell = &self.cells[index];
        //*cell.last_used = tact; //сделать
        cell
    }
    
    pub fn add(&mut self,
        input: &NeuroMatrix,
        output: &NeuroMatrix,
        delta_gain_energy: Option<f32>,
        tact: u32)
    {
        let memorycell = MemoryCell{
            input: input.copy(),
            output: output.copy(),
            delta_gain_energy,
            last_used: tact,
        };
        self.cells.push(memorycell);
        if self.cells.len() > self.capacity{
            // удалить самую старую ячейку
//            todo!("написать код");
//            let result = self.cells.iter().min_by_key(|p| p.last_used);
//            match result{
//                Some(cell) => println!("{}", cell.last_used),
//                None => unreachable!()
//            }
        }
    }
    
    pub fn replace(&mut self,
        index: usize,
        input: &NeuroMatrix,
        output: &NeuroMatrix,
        delta_gain_energy: Option<f32>,
        tact: u32)
    {
        let memorycell = &mut self.cells[index];
        memorycell.input = input.copy();
        memorycell.output = output.copy();
        memorycell.delta_gain_energy = delta_gain_energy;
        memorycell.last_used = tact;
    }
    
    // Возвращает индекс ячейки памяти с ближайшим входом и величину дистанции.
    pub fn find_near(&self, input: &NeuroMatrix) -> Option<(usize, i32)>{
        // индекс ближайшего вектора и величина дистанции.
        self.cells
            .iter()
            .map(|p| p.input.distance(input))
            .enumerate()
            .min_by_key(|(_idx, p)| *p)
    }
    
}



