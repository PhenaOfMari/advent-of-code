use std::collections::HashSet;
use std::fs;
use utils::cartesian::{Cartesian, CARDINALS};
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let garden = input_file.lines()
        .map(|line| line.chars().collect())
        .collect::<Grid<char>>();

    let mut checked_areas = HashSet::new();
    let mut price = 0;
    for i in 0..garden.height() {
        for j in 0..garden.width() {
            let here = Cartesian::new(i as i32, j as i32);
            if checked_areas.get(&here).is_none() {
                let (area, perimeter) = calculate_perimeter(&garden, here, garden.get(here).unwrap(), &mut checked_areas);
                price += area * perimeter;
            }
        }
    }
    println!("{}", price);

    checked_areas = HashSet::new();
    let mut bulk_price = 0;
    for i in 0..garden.height() {
        for j in 0..garden.width() {
            let here = Cartesian::new(i as i32, j as i32);
            if checked_areas.get(&here).is_none() {
                let mut counted_sides = HashSet::new();
                let (area, sides) = calculate_sides(&garden, here, garden.get(here).unwrap(), &mut checked_areas, &mut counted_sides);
                bulk_price += area * sides;
            }
        }
    }
    println!("{}", bulk_price);
}

fn calculate_perimeter(grid: &Grid<char>, here: Cartesian, crop: char, checked: &mut HashSet<Cartesian>) -> (u32, u32) {
    match grid.get(here) {
        Some(current) => {
            if current == crop {
                if checked.insert(here) {
                    CARDINALS.iter().fold((1, 0), |(sum_area, sum_perimeter), &direction| {
                        let (area, perimeter) = calculate_perimeter(grid, here + direction, current, checked);
                        (sum_area + area, sum_perimeter + perimeter)
                    })
                } else {
                    (0, 0)
                }
            } else {
                (0, 1)
            }
        },
        None => {
            (0, 1)
        }
    }
}

fn calculate_sides(grid: &Grid<char>, here: Cartesian, crop: char, checked: &mut HashSet<Cartesian>, counted: &mut HashSet<Cartesian>) -> (u32, u32) {
    match grid.get(here) {
        Some(current) => {
            if current == crop && checked.insert(here) {
                let sides = CARDINALS.iter().fold(0, |sum, &direction| {
                    let mut count_side = grid.get(here + direction).is_none_or(|current| current != crop);
                    let mut check_side = |heading| {
                        let mut spot = here + heading;
                        while count_side
                            && grid.get(spot).is_some_and(|current| current == crop)
                            && grid.get(spot + direction).is_none_or(|current| current != crop) {
                            if counted.contains(&spot) {
                                count_side = false;
                                break;
                            }
                            spot = spot + heading;
                        }
                    };
                    check_side(direction.quarter_turn(true));
                    check_side(direction.quarter_turn(false));
                    if count_side {
                        sum + 1
                    } else {
                        sum
                    }
                });
                counted.insert(here);
                CARDINALS.iter().fold((1, sides), |(sum_area, sum_sides), &direction| {
                    let (area, sides) = calculate_sides(grid, here + direction, current, checked, counted);
                    (sum_area + area, sum_sides + sides)
                })
            } else {
                (0, 0)
            }
        },
        None => {
            (0, 0)
        }
    }
}
