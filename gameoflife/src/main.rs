mod game_of_life;
use std::{thread, time, env};

use rand::Rng;
use std::thread::sleep;

fn main() {

    let mut path = env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_owned();
    path.push_str("\\gospelgun.txt");


    let mut state = game_of_life::get_state_from_file(&path, &'1');

    loop {
        state = game_of_life::next_state(&state);
        println!("{}", game_of_life::render_to_string(&state, (&"\u{2588}", &" ")));
        sleep(time::Duration::from_millis(100));
    }

    println!("Hello, world!");
}


