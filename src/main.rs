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
    
    let mut sol_force_at_osobj = sol.force_at_point(osobj.position);
    println!("Особь в точке {}, освещенность {}", osobj.position, sol_force_at_osobj);
   
    for i in 0..10 {
    
        let signal = osobj.signal_on_sensors(vec![sol_force_at_osobj]);
        println!("сигнал на сенсорах {:?}", signal);
    
        let input = neuronet::Matrix::vec_to_matrix(signal);
        // println!("входной сигнал нейросети {}", input);
    
        let brain_output = osobj.get_brain_output(&input, &sigmoida);
        // println!("выходной сигнал нейросети {}", brain_output);
    
        let common_force = osobj.common_force(&brain_output);
        println!("сумма вектора усилий ног {}", common_force);
    
        osobj.movement(common_force);
        sol_force_at_osobj = sol.force_at_point(osobj.position);
        println!("Особь в точке {}, освещенность {}", osobj.position, sol_force_at_osobj);
    
        osobj.change_energy(sol_force_at_osobj.f);
        println!("Изменение энергии {}", osobj.energy - prev_energy);
        
        prev_energy = osobj.energy;
        
    }
    
}
