// Проект: Жизнь.
//
// Краткое описание: 
// Среда обитания населена особями на основе ИИ. 
// Особи могут поглощать энергию, двигаться, размножаться, умирать.
// Среда обитания имеет набор свойств, благоприятных или неблагоприятных для жизни.
// Свойства могут быть определены сенсорами организмов. 
// Каждый сенсор может воспринимать одно свойство среды. 
// Сенсоры могут быть направленными (воспринимают информацию в определенном направлении) 
// и ненаправленными (воспринимают интегральную информацию вокруг)
// Изначально особи не имеют сенсоров.
// Цель: путем эволюции развить в особях сенсоры и органы движения.

// Благоприятные свойства среды добавляют энергию организму, неблагоприятные - убавляют.
// При накоплении энергии, равной "массе" организма, происходит его деление. 
// Масса - суммарное количество клеток сенсоров, анализатора и органов движения.
// Особь имеет размер, зависящий от массы особи. Минимальное расстояние между особями равно полусумме их линейных разморов (сделать потом)
// Потомство получает слегка модифицированные свойства исходного организма: добавляются или убираются клетки.
// Начальные веса связи нейросети потомков копирует веса нейросети родителя. 
// Для новых клеток веса связи выставляются случайным образом (небольшие величины)

// В ареале обитания имеется источник(и) энергии. Усвояемость энергии обратно пропорциональна квадрату расстояния до источнка.
// При расстоянии, меньшем критического, источник энергии становится ядовитым. Усвояемость энергии уменьшается до нуля, а потом становится отрицательной.
// В ареале обитания имеются источники яда. Ядовитость обратно пропорциональна квадрату расстояни до истоника.
// Имеется постоянный поток, сносящий особи от места максимального усвоения энергии.
// Цель этого: 1) не дать скопиться всем особям в месте максимального усвоения энергии; 2) научить особи двигаться.
// Имеются случайные колебания, рассеивающие особи

// Имеются следующие возможные варианты сенсоров:
//  - Свет
//  - Яд
// Имеются следующие варианты выработки команд:
//  - Двигаться: скорость и направление движения

extern crate neuronet;
use neuronet::NeuroMatrix;
use crate::neuronet::MatrixAdditions;

mod common;
mod memory;
mod osobi;

use common::{Point, Force};
// use osobi::Osobj;

// источник энергии
struct EnergySource {
    position: Point,
    power: f32,
}

impl EnergySource {

    fn force_at_point(&self, p:Point) -> Force{
    
        let dx = p.x - self.position.x;
        let dy = p.y - self.position.y;
        
        let (r, direct) = Point{
            x: dx,
            y: dy,
        }.to_polar();
        
        Force{ 
            // в двумерном мире мощность света обратно пропорциональна расстоянию,
            // при переходе на трехмерный мир переделать на квадрат расстояния
            f: self.power / r, 
            direct: direct,
        }
        
    }
}

fn sample_sol() -> EnergySource {

    let position = Point{
        x: 0.0,
        y: 0.0,
//         z: 0,
    };
    
    EnergySource {
        position: position,
        power: 10000.0,
    }
    
}

// // источник яда
// struct PoisonSource {
//     position: Point,
//     energy: f32,
// }

#[allow(dead_code)]
fn test_signal_on_sensors() {
    
    let sol = sample_sol();
    let mut osobj = osobi::sample_osobj();
     
    osobj.position.y = -100.0;
     
    while osobj.position.x > -120.0 {
     
        let sol_force_at_osobj = sol.force_at_point(osobj.position);
        println!("Особь в точке {}, освещенность {}", osobj.position, sol_force_at_osobj);
        print!("сигнал на сенсорах: ");
        for sensor in osobj.sensors.iter() {
            let signal_at_sensor = sensor.signal_on_sensor(&sol_force_at_osobj);
            print!("{} ", signal_at_sensor);
        }
        println!("");
        
        osobj.position.x = osobj.position.x - 20.0
    
    }

    osobj.position.x = 100.0;
    osobj.position.y = 100.0;
     
    while osobj.position.x > -120.0 {
     
        let sol_force_at_osobj = sol.force_at_point(osobj.position);
        println!("Особь в точке {}, освещенность {}", osobj.position, sol_force_at_osobj);
        print!("сигнал на сенсорах: ");
        for sensor in osobj.sensors.iter() {
            let signal_at_sensor = sensor.signal_on_sensor(&sol_force_at_osobj);
            print!("{} ", signal_at_sensor);
        }
        println!("");
        
        osobj.position.x = osobj.position.x - 20.0
        
    }
    
}

fn delta_gain_energy(new_gain_energy: f32, prev_gain_energy: Option<f32>) -> Option<f32>{
    match prev_gain_energy{
        Some(x) => Some(new_gain_energy - x),
        None => None
    }
}

fn main() {

    // test_signal_on_sensors();
    
    let sigmoida = neuronet::Sigmoida::new();
    
    let sol = sample_sol();
    let mut osobj = osobi::sample_osobj();
    
    //предыдущее накопление энергии
    let mut prev_gain_energy: Option<f32> = None;
    
    let mut sol_force_at_osobj = sol.force_at_point(osobj.position);
    println!("Особь в точке {}, освещенность {}", osobj.position, sol_force_at_osobj);
    
    // Я думаю, надо делать так:
    //  1. Разбить пространство расстояний между векторами входных сигналов Lвх на N частей.
    //      Вычислить среднее расстояние между частями [dLвх(сред)].
    //  2. В окрестности каждого состояния пробовать К раз модифицировать выходной сигнал Lвых.
    //      Определить оптимальный Lвых_опт для максимизации целевой функции (izm_delta_energy).
    //      Записать (Lвх, Lвых_опт) в память.
    //  3. Потренировать нейросеть на Lвых(опт), Lвх (несколько раз?)
    //  4. Потом двигаться на этой нейросети, пока Lвх не удалится от записанных в памяти состояний на [dLвх(сред)]
    //      В этом случае действовать по схеме 2.
    //  5?. Если особь переместится в область [dLвх(мин)] << [dLвх(сред)], то еще раз тренировать нейросеть
    //      на наборе этой ячейки памяти. Чем чаще особь бывает в окрестности записанного состояния, 
    //      тем лучше нужно быть приспособленным именно к этому состоянию.
    //  6. В области [dLвх(мин)] << [dLвх(сред)] время от времени еще раз пробовать модифицировать Lвых
   
    for _ in 0..10 {
    
        let prev_energy = osobj.energy;
        let signal = osobj.signal_on_sensors(vec![sol_force_at_osobj]);
        let input = NeuroMatrix::vec_to_matrix(signal);

        match osobj.find_in_memory(&input) {
        
            osobi::ResultFindInMemory::MakeNewCell => {
            
                println!("ResultFindInMemory::MakeNewCell");
                
                let brain_output = osobj.get_brain_output(&input, &sigmoida);
                let common_force = osobj.common_force(&brain_output);
                osobj.movement(common_force);
                //изменения после перемещения
                sol_force_at_osobj = sol.force_at_point(osobj.position);
                osobj.change_energy(sol_force_at_osobj.f);
                let new_gain_energy = osobj.energy - prev_energy;
                
                // добавляем ячейку памяти
                osobj.add_to_memory(input, 
                    brain_output, 
                    delta_gain_energy(new_gain_energy, prev_gain_energy));
                    
                prev_gain_energy = Some(new_gain_energy);
            },
            
            osobi::ResultFindInMemory::MoveByNeuronet => {
            
                println!("ResultFindInMemory::MoveByNeuronet");
                
                let brain_output = osobj.get_brain_output(&input, &sigmoida);
                let common_force = osobj.common_force(&brain_output);
                osobj.movement(common_force);
                //изменения после перемещения
                sol_force_at_osobj = sol.force_at_point(osobj.position);
                osobj.change_energy(sol_force_at_osobj.f);
                let new_gain_energy = osobj.energy - prev_energy;
                
                prev_gain_energy = Some(new_gain_energy);
            }
            
            osobi::ResultFindInMemory::TryModifyCell(index_memory_cell) => {
            
                // нужно немного модифицировать выходной сигнал, записанный в ячейке памяти
                // и проверить, дала ли эта модификация лучшее накопление энергии, чем записанное в ячейке
                // Если в ячейке не записано изменение накопления энергии, то просто записать новое
                
                println!("ResultFindInMemory::TryModifyCell");
                
                let brain_output;
                let memory_cell_delta_gain_energy;
                {
                    let memory_cell = osobj.get_memory_cell(index_memory_cell);
                    memory_cell_delta_gain_energy = memory_cell.delta_gain_energy;
                    match memory_cell_delta_gain_energy{
                        Some(_) => {
                            brain_output = memory_cell.output.modify(10.0); 
                        },
                        None => {
                            brain_output = osobj.get_brain_output(&input, &sigmoida);
                        }
                    }
                }
                
                let common_force = osobj.common_force(&brain_output);
                osobj.movement(common_force);
                //изменения после перемещения
                sol_force_at_osobj = sol.force_at_point(osobj.position);
                osobj.change_energy(sol_force_at_osobj.f);
                let new_gain_energy = osobj.energy - prev_energy;
                
                match (memory_cell_delta_gain_energy, delta_gain_energy(new_gain_energy, prev_gain_energy)){
                    (Some(x_old), Some(x_new)) => {
                        if x_new > x_old{
                            println!("replaced");
                            osobj.replace_in_memory(index_memory_cell, input, brain_output, Some(x_new));
                        }else{
                            println!("not replaced");
                        }
                    },
                    (None, Some(x_new)) => {
                        println!("replaced first");
                        osobj.replace_in_memory(index_memory_cell, input, brain_output, Some(x_new));
                    }
                    _ => {},
                }
                
                prev_gain_energy = Some(new_gain_energy);
            }

        };
    
                
    }
    
//     osobj.position.x = -100.0;
//     osobj.position.y = -100.0;
//     
//     // тренировка закончена, пускаемся в свободное плавание
//     for _i in 0..50 {
//         let signal = osobj.signal_on_sensors(vec![sol_force_at_osobj]);
//         let input = neuronet::Matrix::vec_to_matrix(signal);
//         let brain_output = osobj.get_brain_output(&input, &sigmoida);
//         //println!("выходной сигнал нейросети {}", brain_output);
//         let common_force = osobj.common_force(&brain_output);
//         osobj.movement(common_force);
//         println!("Особь в точке {}", osobj.position);
//     }
    
}
