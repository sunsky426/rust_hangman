use std::io;

struct Hangman{
    answer: Vec<u8>,
    progress: Vec<bool>,
    life: isize,
}

impl Hangman{
    fn new(answer: String) -> Self {
        let char_list = answer.into_bytes();
        let no_progress = vec![false; char_list.len()];
        Self {
            answer: char_list,
            progress: no_progress,
            life: 6,
        }
    }

    fn guess(&mut self, guess: char) {
        self.life -= 1;

        let guess = guess as u8;
        for i in 0..self.answer.len(){
            if guess == self.answer[i]{
                self.progress[i] = true;
            }
        }
    }

    fn display(&self) {
        println!("{} tries left", self.life);
        let mut toprint: char;

        for i in 0..self.answer.len(){
            if self.progress[i] == true {
                toprint = self.answer[i] as char;
            }else{
                toprint = '_';
            }

            print!("{} ", toprint);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut user_input = String::new();
    let mut game = Hangman::new(String::from("malloc"));
    //let flag = false;

    while !game.progress.iter().all(|x| *x){
        println!("line runs");
        stdin.read_line(&mut user_input).unwrap();
        game.guess(user_input.chars().nth(0).unwrap());
        game.display();
    }

}
