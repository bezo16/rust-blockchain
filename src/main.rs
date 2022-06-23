mod blockchain;
use blockchain::*;




fn main() {
    let mut blockchain = Blockchain::init();
    
    blockchain.add();
    blockchain.add();
    blockchain.add();
    
    println!("{:#?}",blockchain);
}
