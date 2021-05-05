extern crate neuronet;
use neuronet::NeuroMatrix;
// use crate::neuronet::MatrixAdditions;
use crate::neuroadd::MatrixAdditions2;

// Ячейка памяти особи
pub struct MemoryCell {
    pub input: NeuroMatrix,
    pub output: NeuroMatrix,
    // изменение накопления энергии в результате этой реакции input->output
    // по сравнению с накоплением энергии на предыдущем шаге
    pub delta_gain_energy: Option<f32>,
    // проведена тренировка нейросети на этом наборе
    pub n_trained: i16,
}

pub struct Memory {
    pub cells: Vec<MemoryCell>,
}

impl Memory{

    pub fn new() -> Memory {
        Memory{
            cells: Vec::<MemoryCell>::new(),
        }
    }
    
    pub fn get(&self, index: usize) -> &MemoryCell{
        &self.cells[index]
    }
    
    pub fn add(&mut self, input: NeuroMatrix, output: NeuroMatrix, delta_gain_energy: Option<f32>){
        let memorycell = MemoryCell{
            input,
            output,
            delta_gain_energy,
            n_trained: 0,
        };
        self.cells.push(memorycell);
    }
    
    pub fn replace(&mut self, index: usize, input: NeuroMatrix, output: NeuroMatrix, delta_gain_energy: Option<f32>){
        let memorycell = &mut self.cells[index];
        memorycell.input = input;
        memorycell.output = output;
        memorycell.delta_gain_energy = delta_gain_energy;
        memorycell.n_trained = memorycell.n_trained + 1;
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

// #[allow(dead_code)]
// pub fn test_memory_find_near(){
//     
//     let mut memory = Memory::new();
//     
//     let input = Matrix::new_rand(1, 4, 0, 10, false);
//     println!(" 0: {}", &input);
//     let output = Matrix::new(1, 4);
//     memory.add(input, output, 0.0);
// //     
//     let input = Matrix::new_rand(1, 4, 0, 255, false);
//     println!(" n: {}", &input);
//     
//     let result = memory.find_near(&input);
// //     println!("ближайшее: {}", index);
//     match result {
//         Some(x) => println!("ближайшее: индекс {}, расстояние {}", x.0, x.1),//&x.input),
//         None    => println!("ближайшего элемента нет..."),
//     }    
//     
// }



