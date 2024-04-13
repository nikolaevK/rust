
extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

fn main() {
    let snake = snake::Snake::new(1,1);
    println!("Snake order {:#?}", snake.body.front()); 
}
