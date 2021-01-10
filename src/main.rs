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

// источник энергии
struct EnergySource {
    coordinate: common::Coordinates,
    power: i32,
}

fn create_sol() -> EnergySource {

    let coordinate = common::Coordinates{
        x: 0,
        y: 0,
        z: 0,
    };
    
    EnergySource {
        coordinate: coordinate,
        power: 1000,
    }
    
}

// impl EnergySource {
//     fn new(x: f32, y: f32, z: f32, energy: f32) -> EnergySource{
//         EnergySource
//     }
// }

// источник яда
struct PoisonSource {
    coordinate: common::Coordinates,
    energy: f32,
}

fn main() {

    let coordinate = common::Coordinates{
        x: 10,
        y: 20,
        z: 15,
    };
    
    let mut osobj = osobi::Osobj::new(
        coordinate, 
        osobi::simple_brain(),
        osobi::simple_sensors(),
        osobi::simple_legs(),
        100
    );

//     let light1 = EnergySource{
//         coordinate: Coordinates{
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//         },
//         energy: 100.0
//     };
//     
//     let mut nloop = 0;
//     loop {
//         nloop = nloop + 1;
//         if nloop > 1000 {
//             break
//         }
//         
//         
//     };
    

}
