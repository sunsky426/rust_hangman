mod game_data;

use std::{io, path::Path, fs};
use rand::{Rng, thread_rng};
use std::process::Command;
use game_data::{GameData, GameStatus};

fn main() {
    let stdin = io::stdin();
    let mut game = GameData::new(choose_answer(Path::new("./src/words.txt")));

    clear();
    loop{
        game.display();

        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();
        match game.game_status{
            GameStatus::Victory | GameStatus::Defeat => break,
            GameStatus::InProgress => {},
        }
        clear();

        game.guess(user_input.chars().nth(0).unwrap());
        game.update_gamestatus();
    }
}

fn clear(){
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    }else{
        Command::new("clear").status().unwrap();
    }
}

fn choose_answer(path: &Path) -> String{
    let content = fs::read_to_string(path).expect("Can not read file");
    let content:Vec<&str> = content.lines().collect();
    let index = thread_rng().gen_range(1..content.len());
    content[index].to_owned()
}