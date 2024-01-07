mod guess;


fn main() {
    let mut guess_game : guess::GuessGame = guess::GuessGame::new((1, 101));
    if guess_game.start() {
        println!("good game!");
    }
}
