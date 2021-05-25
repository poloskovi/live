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
use neuronet::{NeuroMatrix, Sigmoida};

mod common;
mod memory;
mod osobi;
mod neuroadd;
use crate::neuroadd::MatrixAdditions2;

use common::{Point, Polar, Force, Direct};

// отображение
use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;
use plotters_piston::{draw_piston_window, PistonBackend};

// источник энергии
struct EnergySource {
    position: Polar,
    power: f32,
}

impl EnergySource {

    fn force_at_point(&self, p:Point) -> Force{
    
        let sol_point = self.position.to_decart();
        let dx = p.x - sol_point.x;
        let dy = p.y - sol_point.y;
        
        let polar = Point{
            x: dx,
            y: dy,
        }.to_polar();

        Force{ 
            // в двумерном мире мощность света обратно пропорциональна расстоянию,
            // при переходе на трехмерный мир переделать на квадрат расстояния
            f: self.power / polar.r,
            direct: polar.direct,
        }
        
    }
}

fn sample_sol() -> EnergySource {
    let position = Polar::new(900.0, Direct::new(0.0));
    EnergySource {
        position: position,
        power: 100000.0,
    }
}

// // источник яда
// struct PoisonSource {
//     position: Point,
//     energy: f32,
// }

fn delta_gain_energy(new_gain_energy: f32, prev_gain_energy: Option<f32>) -> Option<f32>{
    match prev_gain_energy{
        Some(x) => Some(new_gain_energy - x),
        None => None
    }
}

fn osobj_movement(tact: u32,
    osobj: &mut osobi::Osobj,
    sol: &EnergySource,
    sigmoida: &Sigmoida){

    let mut sol_force_at_osobj = sol.force_at_point(osobj.position);
    let prev_energy = osobj.energy;
    let signal = osobj.signal_on_sensors(vec![sol_force_at_osobj]);
    let input = NeuroMatrix::vec_to_matrix(signal);

    match osobj.find_in_memory(&input) {

        osobi::ResultFindInMemory::MakeNewCell => {

            let brain_output = osobj.get_brain_output(&input, sigmoida);
            let common_force = osobj.common_force(&brain_output);
            osobj.movement(common_force);

            //изменения после перемещения
            sol_force_at_osobj = sol.force_at_point(osobj.position);
            osobj.change_energy(sol_force_at_osobj.f);
            let new_gain_energy = osobj.energy - prev_energy;

            // добавляем ячейку памяти
            osobj.add_to_memory(&input,
                &brain_output,
                delta_gain_energy(new_gain_energy, osobj.prev_gain_energy),
                tact
            );

            osobj.prev_gain_energy = Some(new_gain_energy);
        },

        osobi::ResultFindInMemory::MoveByNeuronet => {

            if osobj.need_train_brain {
                osobj.brain_training(sigmoida);
                osobj.need_train_brain = false;
            };

            let brain_output = osobj.get_brain_output(&input, sigmoida);
            let common_force = osobj.common_force(&brain_output);
            osobj.movement(common_force);
            //изменения после перемещения
            sol_force_at_osobj = sol.force_at_point(osobj.position);
            osobj.change_energy(sol_force_at_osobj.f);
            let new_gain_energy = osobj.energy - prev_energy;

            osobj.prev_gain_energy = Some(new_gain_energy);
        }

        osobi::ResultFindInMemory::TryModifyCell(index_memory_cell) => {

            // нужно немного модифицировать выходной сигнал, записанный в ячейке памяти
            // и проверить, дала ли эта модификация лучшее накопление энергии, чем записанное в ячейке
            // Если в ячейке не записано изменение накопления энергии, то просто записать новое

            let brain_output;
            let memory_cell_delta_gain_energy;
            {
                let memory_cell = osobj.get_memory_cell(index_memory_cell, tact);

                memory_cell_delta_gain_energy = memory_cell.delta_gain_energy;
                match memory_cell_delta_gain_energy{
                    Some(_) => {
                        brain_output = memory_cell.output.modify(30.0);
                    },
                    None => {
                        brain_output = osobj.get_brain_output(&input, sigmoida);
                    }
                }
            }

            let common_force = osobj.common_force(&brain_output);
            osobj.movement(common_force);
            //изменения после перемещения
            sol_force_at_osobj = sol.force_at_point(osobj.position);
            osobj.change_energy(sol_force_at_osobj.f);
            let new_gain_energy = osobj.energy - prev_energy;

            match (memory_cell_delta_gain_energy, delta_gain_energy(new_gain_energy, osobj.prev_gain_energy)){
                (Some(x_old), Some(x_new)) => {
                    if x_new > x_old{
                        osobj.replace_in_memory(
                            index_memory_cell,
                            &input,
                            &brain_output,
                            Some(x_new),
                            tact
                        );
                    }
                },
                (None, Some(x_new)) => {
                    osobj.replace_in_memory(
                        index_memory_cell,
                        &input,
                        &brain_output,
                        Some(x_new),
                        tact
                    );
                }
                _ => {},
            }

            osobj.prev_gain_energy = Some(new_gain_energy);
        }

    }// результат поиска в памяти

}

fn to_window_coord(p: Point, window_center: (i32, i32), scale: f32) -> (i32, i32){
    ((window_center.0 + (p.x * scale) as i32),
      window_center.1 - (p.y * scale) as i32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // окно отрисовки
    let mut window: PistonWindow = WindowSettings::new("Live", [450, 450])
        .samples(4)
        .build()
        .unwrap();

    // используется в нейросети
    let sigmoida = neuronet::Sigmoida::new();
    
    // солнце - источник энергии
    let mut sol = sample_sol();

    // особи
    let mut m_osobi = Vec::new();

    // наш Адам. Или Ева
    m_osobi.push(osobi::sample_osobj());

    //let mut osobj = osobi::sample_osobj();

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
   
    let mut tact = 0;
    while let Some(_) = draw_piston_window(&mut window, |b| {

        let root = b.into_drawing_area();
        root.fill(&WHITE)?;
        let window_size = root.dim_in_pixel(); // размер окна
        //println!{"{:?}", window_size};
        let window_center = ((window_size.0/2) as i32, (window_size.1/2) as i32); // координаты центра окна

        let scale = (window_size.1 / 2) as f32 / (900.0 * 1.2); // 900 - радиус орбиты Солнца

        // Аквариум. Определен в osobi.
        // Переделать определение конфигурации аквариума в отдельный объект
        root.draw(&Circle::new(
                window_center,
                (400.0 * scale) as i32,
                Into::<ShapeStyle>::into(&BLUE),
            ))?;

        sol.position.direct.fi = sol.position.direct.fi + 0.005; // в градусах
        if sol.position.direct.fi > 360.0{
            sol.position.direct.fi = sol.position.direct.fi - 360.0;
        };

        // рисуем солнце
        let solar_coord = to_window_coord(
            sol.position.to_decart(),
            window_center,
            scale);
        //println!{"{:?}", solar_coord};
        root.draw(&Circle::new(
                solar_coord,
                (30.0 * scale) as i32,
                Into::<ShapeStyle>::into(&YELLOW).filled(),
            ))?;


        let mut need_add = Vec::new();
        for osobj in m_osobi.iter_mut(){
            if !osobj.dead{
                if osobj.energy > osobj.massa{
                    //println!("Деление");
                    need_add.push(osobj.copy_modify());
                    osobj.energy = osobj.energy / 2.0;
                }else if osobj.energy < 0.0{
                    //println!("Особь погибла");
                    osobj.dead = true;
                }
            }
        }
        for osobj in need_add{
            m_osobi.push(osobj);
        }

        for osobj in m_osobi.iter_mut(){

            if osobj.dead{
                continue
            }


            osobj_movement(tact, osobj, &sol, &sigmoida);

            let osobj_coord = to_window_coord(
                osobj.position,
                window_center,
                scale);
            //println!{"{:?}", solar_coord};
            root.draw(&Circle::new(
                    osobj_coord,
                    (8.0 * scale) as i32,
                    Into::<ShapeStyle>::into(&BLACK).filled(),
                ))?;


        }// особи

        tact += 1;
    
        Ok(())
    }){};
    Ok(())

//    println!(" живых {} особей:", m_osobi.len());
//    for osobj in m_osobi.iter(){
//        println!("{:?}", osobj.nnodes);
//    }
    
//    println!("ячейки памяти:");
//    for memorycell in osobj.memory.cells.iter(){
//        println!("{}", memorycell.last_used);
//        print!("{}", &memorycell.input);
//        println!("{}", &memorycell.output);
//    }

}

//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
//    root.fill(&WHITE)?;
//    let mut chart = ChartBuilder::on(&root)
//        .caption("y=x^2", ("sans-serif", 50).into_font())
//        .margin(5)
//        .x_label_area_size(30)
//        .y_label_area_size(30)
//        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

//    chart.configure_mesh().draw()?;

//    chart
//        .draw_series(LineSeries::new(
//            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
//            &RED,
//        ))?
//        .label("y = x^2")
//        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

//    chart
//        .configure_series_labels()
//        .background_style(&WHITE.mix(0.8))
//        .border_style(&BLACK)
//        .draw()?;

//    main_2();

//    Ok(())
//}
