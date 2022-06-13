mod lib;
use clock::Clock;

fn main(){
    println!("{}", Clock::new(0, 1441));
    println!("{}", Clock::new(2, 4322));
    println!("{}", Clock::new(0, 1723));

}