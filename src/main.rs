use std::io;
use std::process::Command;

const HANGINGMAN: &'static [&'static str] = &[
    "+-----+\n|     |\n|     0\n|    /|\\ \n|    / \\ \n|\n===+=======",
    "+-----+\n|     |\n|     0\n|    /|\\ \n|    /\n|\n===+=======",
    "+-----+\n|     |\n|     0\n|    /|\\ \n|\n|\n===+=======",
    "+-----+\n|     |\n|     0\n|    /|\n|\n|\n===+=======",
    "+-----+\n|     |\n|     0\n|     |\n|\n|\n===+=======",
    "+-----+\n|     |\n|     0\n|\n|\n|\n===+=======",
    "+-----+\n|     |\n|\n|\n|\n|\n===+=======",
    "+-----+\n|\n|\n|\n|\n|\n===+=======",
    "\n|\n|\n|\n|\n|\n===+=======",
    "\n\n\n\n\n\n===+=======",
    "\n\n\n\n\n\n"
    ];

enum GameState{
    InProgress,
    Victory,
    Defeat
}
struct Hangman{
    answer: Vec<char>,
    progress: Vec<bool>,
    life: usize,
}

impl Hangman{
    fn new(answer: String) -> Self {
        let char_list: Vec<char> = answer.chars().collect();
        let no_progress = vec![false; char_list.len()];
        Self {
            answer: char_list,
            progress: no_progress,
            life: 10,
        }
    }

    #[allow(non_snake_case)]
    fn guess(&mut self, guess: char){
        let mut isAccurateGuess = false;

        for i in 0..self.answer.len(){
            if guess == self.answer[i]{
                self.progress[i] = true;
                isAccurateGuess = true
            }
        }

        if !isAccurateGuess{
            self.life -= 1; 
        }
    }

    fn display(&self) {
        println!("{}", HANGINGMAN[self.life]);

        for i in 0..self.answer.len(){
            if self.progress[i] == true {
                print!("{} ", self.answer[i]);
            }else{
                print!("{} ", '_');
            }
        }
    println!("")
    }

    fn update_gamestate(&self) -> GameState{
        if self.progress.iter().all(|x| *x){
            println!("You Won");
            return GameState::Victory;
        }else if self.life == 0{
            println!("GAME OVER");
            return GameState::Defeat;
        }else{
            return GameState::InProgress;
        }
    }
}

fn main() {
    let mut game_state = GameState::InProgress;
    let stdin = io::stdin();
    let mut game = Hangman::new(String::from("malloc"));

    clear();
    loop{
        game.display();

        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();
        match game_state{
            GameState::Victory | GameState::Defeat => break,
            GameState::InProgress => {},
        }
        clear();

        game.guess(user_input.chars().nth(0).unwrap());
        game_state = game.update_gamestate();
    }
}

fn clear(){
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    }else{
        Command::new("clear").status().unwrap();
    }
}
