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

mod common;
mod neuronet;
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


fn main() {

    // test_signal_on_sensors();
    
    let sigmoida = neuronet::Sigmoida::new();
    
    let sol = sample_sol();
    let mut osobj = osobi::sample_osobj();
    
    let mut prev_energy = osobj.energy;
    let mut delta_energy: f32;
    let mut prev_delta_energy: Option<f32> = None;
    let mut izm_delta_energy: f32;
    
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
   
    for _i in 0..300 {
    
//         println!("");
//         println!("шаг {}", _i);
    
        let signal = osobj.signal_on_sensors(vec![sol_force_at_osobj]);
        //println!("сигнал на сенсорах {:?}", signal);
    
        let input = neuronet::Matrix::vec_to_matrix(signal);
        // println!("входной сигнал нейросети {}", input);
        
        // нужно анализировать изменение прироста энергии
    
        let mut in_memory = osobj.find_in_memory(&input);
        
        match in_memory {
            Some(x) => {
                let index_memory_cell = x.0;
                let length_to_memory_input = x.1;
                if length_to_memory_input > 5 {
                    // отошли далеко от записанных в памяти состояний
                    // Тренируем нейросеть на последнем состоянии
                    // вместо последнего состояния пока берем ближайшее
                    let memory_cell = osobj.get_memory_cell(index_memory_cell);
                    let input_copy = memory_cell.input.copy();
                    let output_copy = memory_cell.output.copy();
                    osobj.brain_training(&input_copy, &output_copy, &sigmoida);
                    println!("Сеть потренерована на ячейке памяти {}", index_memory_cell);
                    in_memory = None;
                }
            },
            None => {},
        }
        
        match in_memory {
            Some(x) => {
            
                let index_memory_cell = x.0;
                let length_to_memory_input = x.1;
//                 println!("ближайшее: индекс {}, расстояние {}", index_memory_cell, length_to_memory_input);
                
                let brain_output;
                let memory_cell_izm_delta_energy;
                {
                    let memory_cell = osobj.get_memory_cell(index_memory_cell);
                    memory_cell_izm_delta_energy = memory_cell.izm_delta_energy;
                    brain_output = memory_cell.output.modify(20.0);
                }
                //println!("модифицированный выходной сигнал нейросети {}", brain_output);
                
                let common_force = osobj.common_force(&brain_output);
                //println!("сумма вектора усилий ног {}", common_force);
                
                osobj.movement(common_force);
                sol_force_at_osobj = sol.force_at_point(osobj.position);
//                 println!("Особь в точке {}, освещенность {}", osobj.position, sol_force_at_osobj);
    
                osobj.change_energy(sol_force_at_osobj.f);
                
                delta_energy = osobj.energy - prev_energy;
                match prev_delta_energy{
                    Some(x) => {
                        izm_delta_energy = delta_energy - x;
//                         println!("Изменение накопления энергии {}", izm_delta_energy);
                
                        if izm_delta_energy > memory_cell_izm_delta_energy{
//                             println!("эта модификация ЛУЧШЕ! {} {}", izm_delta_energy, memory_cell_izm_delta_energy);
//                             println!(" == Меняем запись в ячейке памяти");
                            osobj.replace_in_memory(index_memory_cell, input, brain_output, izm_delta_energy);
                        }else{
//                             println!("эта модификация хуже... {} {}", izm_delta_energy, memory_cell_izm_delta_energy);
                        }
                    },
                    None =>{},
                };
        
            },
            None    => {
            
//                 println!("ближайшего элемента нет...");
                
                let brain_output = osobj.get_brain_output(&input, &sigmoida);
//                 println!("выходной сигнал нейросети {}", brain_output);
    
                let common_force = osobj.common_force(&brain_output);
//                 println!("сумма вектора усилий ног {}", common_force);
    
                osobj.movement(common_force);
                sol_force_at_osobj = sol.force_at_point(osobj.position);
//                 println!("Особь в точке {}, освещенность {}", osobj.position, sol_force_at_osobj);
    
                osobj.change_energy(sol_force_at_osobj.f);
                
                delta_energy = osobj.energy - prev_energy;
                match prev_delta_energy {
                    Some(x) => {
                        izm_delta_energy = delta_energy - x;
//                         println!("Изменение накопления энергии {}", izm_delta_energy);
                        osobj.put_to_memory(input, brain_output, izm_delta_energy);
                    },
                    None => {},
                };
        
            },

        };
    
//         println!("delta_energy={}", delta_energy);
        prev_delta_energy = Some(delta_energy);
        prev_energy = osobj.energy;
                
    }
    
    osobj.position.x = -100.0;
    osobj.position.y = -100.0;
    
    // тренировка закончена, пускаемся в свободное плавание
    for i in 0..50 {
        let signal = osobj.signal_on_sensors(vec![sol_force_at_osobj]);
        let input = neuronet::Matrix::vec_to_matrix(signal);
        let brain_output = osobj.get_brain_output(&input, &sigmoida);
        //println!("выходной сигнал нейросети {}", brain_output);
        let common_force = osobj.common_force(&brain_output);
        osobj.movement(common_force);
        println!("Особь в точке {}", osobj.position);
    }
    
}
