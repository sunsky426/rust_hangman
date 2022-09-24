use std::io;

struct Hangman{
    answer: Vec<char>,
    progress: Vec<bool>,
    life: isize,
}

impl Hangman{
    fn new(answer: String) -> Self {
        let char_list: Vec<char> = answer.chars().collect();
        let no_progress = vec![false; char_list.len()];
        Self {
            answer: char_list,
            progress: no_progress,
            life: 6,
        }
    }

    fn guess(&mut self, guess: char) {
        self.life -= 1;

        for i in 0..self.answer.len(){
            if guess == self.answer[i]{
                self.progress[i] = true;
            }
        }
    }

    fn display(&self) {
        println!("{} tries left", self.life);

        for i in 0..self.answer.len(){
            if self.progress[i] == true {
                print!("{} ", self.answer[i]);
            }else{
                print!("{} ", '_');
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut game = Hangman::new(String::from("malloc"));

    while !game.progress.iter().all(|x| *x){
        println!("");
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();

        game.guess(user_input.chars().nth(0).unwrap());
        game.display();
    }

}
