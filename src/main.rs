mod character;
mod cooking;
mod level;

use crate::character::Character;
use plotters::prelude::*;
use std::collections::HashMap;

fn mode_count(v: &Vec<u32>) -> u32 {
    let mut occurrences = HashMap::new();

    for &value in v {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .1
}

fn get_burnt_shrimp_count() -> usize {
    let mut character = Character::after_tutorial_island();

    while character.cooking_level() < 34 {
        character.cook_one_shrimp();
    }

    character.cooking_stats.burnt_shrimp
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = 200000;

    println!("Generating {} burnt shrimp counts...", count);
    let data: Vec<_> = (0..count)
        .map(|_| get_burnt_shrimp_count() as u32)
        .collect();

    println!("Drawing bucket plot");
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("# of accounts that burnt X shrimp", ("sans-serif", 40))
        .build_cartesian_2d(
            (*data.iter().min().unwrap()..*data.iter().max().unwrap() + 1).into_segmented(),
            0u32..(mode_count(&data)),
        )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("# of Accounts")
        .x_desc("Shrimps Burnt")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .margin(0)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x| (*x, 1))),
    )?;

    println!("Drawing pdf plot");
    let root = BitMapBackend::new("pdf.png", (800, 600)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(50)
        .margin(10)
        .caption("probability of burning X shrimp", ("sans-serif", 40))
        .build_cartesian_2d(
            *data.iter().min().unwrap()..*data.iter().max().unwrap(),
            0f32..100f32,
        )?;

    chart
        .configure_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("% of accounts that burnt at most X shrimp")
        .x_desc("Shrimps Burnt")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(AreaSeries::new(
        (*data.iter().min().unwrap()..*data.iter().max().unwrap() + 1).map(|x| {
            let below = data.iter().filter(|y| **y <= x).count() as f32;
            (x, below * 100.0 / count as f32)
        }),
        0.0,
        &RED.mix(0.2),
    ))?;

    Ok(())
}
