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
    
pub enum GameStatus{
    InProgress,
    Victory,
    Defeat
}
pub struct GameData{
    pub answer: Vec<char>,
    pub progress: Vec<bool>,
    pub life: usize,
    pub game_status: GameStatus,
}

impl GameData{
    pub fn new(answer: String) -> Self {
        let char_list: Vec<char> = answer.chars().collect();
        let no_progress = vec![false; char_list.len()];
        Self {
            answer: char_list,
            progress: no_progress,
            life: 10,
            game_status: GameStatus::InProgress,
        }
    }

    #[allow(non_snake_case)]
    pub fn guess(&mut self, guess: char){
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

    pub fn display(&self) {
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

    pub fn update_gamestatus(&mut self) {
        if self.progress.iter().all(|x| *x){
            println!("You Won");
            self.game_status = GameStatus::Victory;
        }else if self.life == 0{
            println!("GAME OVER");
            self.game_status = GameStatus::Defeat;
        }
    }
}