use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::path::Path;


pub fn render_to_string(state: &Vec<Vec<i32>>, (alive_c, dead_c): (&str, &str)) -> String {

    let str_vec = state.iter()
                              .map(|x| x.iter()
                                                     .map(|&x| match x
                                                                        {
                                                                            1 => alive_c,
                                                                            _ => dead_c
                                                                        }).collect::<Vec<&str>>().join(" ")
                                                    ).collect::<Vec<String>>().join("\n");
    str_vec
}

pub fn get_state_from_file(path: &str, &alive_c: &char) -> Vec<Vec<i32>> {
    let file = File::open(path).unwrap();

    let buf_reader = BufReader::new(file);

    let lines = buf_reader.lines();

     let state = lines.into_iter().map(|x| x.unwrap().chars().map(|y| match y {
         y if y == alive_c => 1,
         _ => 0
     }).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    state
}

pub fn random_state(width:usize, height: usize, proportion: f64) -> Vec<Vec<i32>>{

    let mut rng = rand::thread_rng();

    let mut rand_state = vec![vec![0; width]; height];

    for row in rand_state.iter_mut(){
        for i in row.iter_mut(){
            let rng: f64 = rng.gen();
            *i = if rng > proportion {1} else {0}

        }
    }

    rand_state
}


fn get_neighbors_alive(s: &Vec<Vec<i32>>, i1: i32, i2: i32) ->  i32 {

    let h = s.len() as i32;
    let w = s[0].len() as i32;

    let tup = vec![(i1 + 1, i2), (i1 - 1, i2), (i1, i2 + 1), (i1, i2 - 1), (i1 + 1, i2 + 1), (i1 - 1, i2 - 1), (i1 + 1, i2 - 1), (i1 - 1, i2 + 1)];

    let alive = tup.iter()
        .fold(0, |a, &(y, x)|
            if (y >= 0 && y < h) && (x >= 0 && x < w) && (s[y as usize][x as usize]  == 1)
                {a + 1}
            else
                {a});
    alive
}

pub fn next_state(initial_state: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_state = vec![vec![0; initial_state[0].len()]; initial_state.len()];
    for (i, row) in initial_state.iter().enumerate(){
        for (j, col) in row.iter().enumerate(){
            let alive = get_neighbors_alive(initial_state, i as i32, j as i32);
            new_state[i ][j] = if *(col) == 0
            { if alive == 3 { 1 } else{0} }
            else
            { if alive > 3 || alive <= 1 { 0 } else {1} }
        }
    }

    new_state
}