mod lib;
use clock::Clock;

fn main(){
    println!("{}", Clock::new(8, 0));
    println!("{}", Clock::new(11, 9));

}