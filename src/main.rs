mod game_data;

use std::io;
use std::process::Command;
use game_data::GameData;
use game_data::GameStatus;

fn main() {
    let stdin = io::stdin();
    let mut game = GameData::new(String::from("malloc"));

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
