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
    fn guess(&mut self, guess: char) -> bool{
        let mut isAccurateGuess = false;

        for i in 0..self.answer.len(){
            if guess == self.answer[i]{
                self.progress[i] = true;
                isAccurateGuess = true
            }
        }

        if !isAccurateGuess{
            if self.life > 0{
                self.life -= 1;
            }else{
                println!("GAME OVER");
                return true;
            }    
        }
        return false;
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
}

fn main() {
    let mut game_over = false;
    let stdin = io::stdin();
    let mut game = Hangman::new(String::from("malloc"));

    clear();
    while !game.progress.iter().all(|x| *x){
        game.display();

        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();
        clear();
        if game_over{
            break;
        }

        game_over = game.guess(user_input.chars().nth(0).unwrap());
    }
}

fn clear(){
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    }else{
        Command::new("clear").status().unwrap();
    }
}
