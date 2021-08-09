use a3::block::Block;

fn main() {
    // Nothing is required here, but it may be useful for testing.
    // let mut b0 = Block::initial(19);
    // b0.set_proof(87745);
    // let mut b1 = Block::next(&b0, String::from("hash example 1234"));
    // b1.set_proof(1407891);
    // println!("{:x}", b0.hash());
    // println!("{:x}", b1.hash());
    // println!("{:x}", b1.hash());
    // println!("{}", b0.is_valid_for_proof(87745));
    // println!("{}", b1.is_valid_for_proof(346082));
    let mut b0 = Block::initial(20);
    b0.mine(8);
    println!("{}", b0.hash_string());
    println!("{:02x}", b0.hash());
    let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
    b1.mine(8);
    println!("{}", b1.hash_string());
    println!("{:02x}", b1.hash());
    let mut b2 = Block::next(&b1, String::from("this is not interesting"));
    b2.mine(8);
    println!("{}", b2.hash_string());
    println!("{:02x}", b2.hash());
}

