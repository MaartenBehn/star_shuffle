
use std::{thread, result, sync::mpsc::{self, Sender}, time::TryFromFloatSecsError};
use itertools::Itertools;

const STAR_SIZE: usize = 12;

const UPPER_BOUND: usize = 42;
const LOWER_BOUND: usize = 0;


fn main() {
    // line_search();
    // coner_search();
}

fn line_search(){
    let mut original_star = [0; STAR_SIZE];

    for i in 0..STAR_SIZE {
        original_star[i] = i + 1;
    }

    for i in 1..STAR_SIZE + 1 {

        println!("Changed Notes: {:?}", i);

        let mut handles = Vec::new();
        for j in 0..STAR_SIZE - i + 1 {
           
            let star = original_star;
            let handle = thread::spawn(move || {
                shuffle_star(star, j, i + j - 1);
            });

            handles.push(handle);
            
        }

        for handle in handles {
            handle.join();
        }
    }
}


fn coner_search() {
    let mut original_star = [0; STAR_SIZE];

    for i in 0..STAR_SIZE {
        original_star[i] = i + 1;
    }

    let star = original_star;

    let mut results = Vec::new();

    shuffle_corners(star, 0, &mut results);

    let mut unique_results = results.into_iter().unique().collect::<Vec<_>>();

    unique_results.sort_by(|a, b| {
        let ch_a = get_changed_nodes(a, &original_star);
        let ch_b = get_changed_nodes(b, &original_star);

        ch_a.cmp(&ch_b)
    });

    for result in unique_results {
        println!("Checking: {:?}", result);

        shuffle_corner_lines(result, 0, &original_star);
    }
}

fn shuffle_star(mut star: [usize; STAR_SIZE], current: usize, max_depth: usize){

    for i in LOWER_BOUND..UPPER_BOUND {

        star[current] = i;

        if check_lines(star){

            if check_corners(star) {
                println!("LINE AND CORNERS: {:?}", star);
            }
            else{
                println!("LINE: {:?}", star);
            }
        }

        if current < max_depth {
            shuffle_star(star, current + 1, max_depth);
        }
    }
}

fn shuffle_corners(mut star: [usize; STAR_SIZE], corner_index: usize, results: &mut Vec<[usize; STAR_SIZE]>) {
    let corner_indices = [1, 4, 11, 9, 3, 2];

    for i in LOWER_BOUND..UPPER_BOUND {

        star[corner_indices[corner_index] - 1] = i;

        if check_corners(star){
            println!("CORNERS: {:?}", star);

            results.push(star);
        }

        if corner_index < 5 {
            shuffle_corners(star, corner_index + 1, results);
        }
    }
}

fn shuffle_corner_lines(mut star: [usize; STAR_SIZE], inner_index: usize, original_star: &[usize]){
    let innner_indices = [0, 5, 6, 7, 8, 10];

    for i in LOWER_BOUND..UPPER_BOUND {

        star[innner_indices[inner_index]] = i;

        if check_lines(star){
            let changed = get_changed_nodes(&star, original_star);
            println!("CORNERS AND LINES: {:?} {:?}", star, changed);
        }

        if inner_index < 5 {
            shuffle_corner_lines(star, inner_index + 1, original_star);
        }
    }
}

const LINE_SUM: usize = 42;
fn check_lines(star: [usize; STAR_SIZE]) -> bool {
    let mut lines = [false; 6];

    lines[0] = (star[2 - 1] + star[12 - 1] + star[8 - 1] + star[4 - 1] == LINE_SUM);
    lines[1] = (star[1 - 1] + star[8 - 1] + star[6 - 1] + star[11 - 1] == LINE_SUM);
    lines[2] = (star[4 - 1] + star[6 - 1] + star[7 - 1] + star[9 - 1] == LINE_SUM);
    lines[3] = (star[11 - 1] + star[7 - 1] + star[5 - 1] + star[3 - 1] == LINE_SUM);
    lines[4] = (star[9 - 1] + star[5 - 1] + star[10 - 1] + star[2 - 1] == LINE_SUM);
    lines[5] = (star[3 - 1] + star[10 - 1] + star[12 - 1] + star[1 - 1] == LINE_SUM);

    lines[0] && lines[1] && lines[2] && lines[3] && lines[4] && lines[5]
}

const CORNER_SUM: usize = 42;
fn check_corners(star: [usize; STAR_SIZE]) -> bool {
    star[1 - 1] + star[4 - 1] + star[11 - 1] + star[9 - 1] + star[3 - 1] + star[2 - 1] == CORNER_SUM
}

fn get_changed_nodes(star: &[usize], original_star: &[usize]) -> usize {

    let mut changed = 0;
    for (i, node) in star.iter().enumerate() {
        if *node != original_star[i] {
            changed += 1;
        }
    }

    return changed;
}