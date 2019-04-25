use rand::Rng;
use old_rand::Rng as OldRng;

fn main() {
    println!("new version: {}", rand::thread_rng().gen::<u8>());
    println!("old version: {}", old_rand::thread_rng().gen::<u8>());
}
