use ctsim::{create_game, run_game};

fn main() {
    println!("hello");

    let mut g = create_game();
    run_game(&mut g);

    // let mut vec = vec![3, 4, 5];
    // let slice = vec![0, 1, 2];

    // vec.splice(0..0, slice);

    // println!("{:?}", vec); // [1, 2, 3, 4, 5]
}
