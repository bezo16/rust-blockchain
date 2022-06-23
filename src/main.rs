mod blockchain;
use blockchain::*;



fn main() {
    let blockchain = Blockchain::init();
    println!("{:?}",blockchain)
}
