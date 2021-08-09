#[cfg(test)]
mod block_tests {
    use crate::block::{Block};
    use ntest::timeout;

    #[test]
    fn example() {
        assert_eq!(1+1, 2);
    }

    #[test]
    fn chained_correctly(){
        let n_threads: usize = 2;

        let mut b0 = Block::initial(8);
        assert_eq!(b0.proof, None);
        b0.mine(n_threads);
        assert_ne!(b0.proof, None);
        let mut b1 = Block::next(&b0, String::from("Assignment is so hard."));
        assert_eq!(b1.proof, None);
        assert_eq!(b1.prev_hash, b0.hash());
        b1.mine(n_threads);
        assert_ne!(b1.proof, None);
        let mut b2 = Block::next(&b1, String::from("I don't know what I am doing"));
        assert_eq!(b2.proof, None);
        assert_eq!(b2.prev_hash, b1.hash());
        b2.mine(n_threads);
        assert_ne!(b2.proof, None);
    }

    #[test]
    fn correct_proof(){
        let n_threads: usize = 2;
        let mut b0 = Block::initial(8);
        b0.mine_serial();
        let mut b1 = Block::next(&b0, String::from("My code doesn't work"));
        b1.mine_serial();
        let mut b2 = Block::next(&b1, String::from("I am super sad!"));
        b2.mine_serial();

        let correct_proof = b2.proof.unwrap();

        let mut b3 = Block::initial(8);
        b3.mine(n_threads);
        let mut b4 = Block::next(&b3, String::from("My code doesn't work"));
        b4.mine(n_threads);
        let mut b5 = Block::next(&b4, String::from("I am super sad!"));
        b5.mine(n_threads);

        let my_proof = b5.proof.unwrap();

        assert_eq!(correct_proof, my_proof);
    }

    #[test]
    fn concurrent_correctly(){
        let mut n_threads: usize = 2;
        let mut b0 = Block::initial(8);
        b0.mine(n_threads);
        let mut b1 = Block::next(&b0, String::from("I tried to fix it."));
        b1.mine(n_threads);
        let mut b2 = Block::next(&b1, String::from("But I failed."));
        b2.mine(n_threads);

        let proof_with_2_worker = b2.proof.unwrap();

        n_threads = 4;

        let mut b3 = Block::initial(8);
        b3.mine(n_threads);
        let mut b4 = Block::next(&b3, String::from("I tried to fix it."));
        b4.mine(n_threads);
        let mut b5 = Block::next(&b4, String::from("But I failed."));
        b5.mine(n_threads);

        let proof_with_4_worker = b5.proof.unwrap();

        assert_eq!(proof_with_2_worker, proof_with_4_worker);
    }

    #[test]
    fn range_inbound(){
        let n_threads: usize = 2;

        let mut b0 = Block::initial(7);
        b0.mine_serial();
        let mut b1 = Block::next(&b0, String::from("why my code doesn't work?"));
        b1.mine_serial();
        let mut b2 = Block::next(&b1, String::from("why??"));
        b2.mine_serial();

        let start = 0;
        let end = b2.proof.unwrap() + 1;
        let chunk = (end - start) / 4;

        let proof = b2.mine_range(n_threads, start, end, chunk);
        assert_eq!(proof, b2.proof.unwrap());
    }

    #[test]
    #[timeout(1000)]
    #[should_panic]
    fn range_outbound(){
        let n_threads: usize = 2;

        let mut b0 = Block::initial(7);
        b0.mine_serial();
        let mut b1 = Block::next(&b0, String::from("why my code doesn't work?"));
        b1.mine_serial();
        let mut b2 = Block::next(&b1, String::from("why??"));
        b2.mine_serial();

        let start = 0;
        let end = b2.proof.unwrap();
        let chunk = (end - start) / 4;

        let proof = b2.mine_range(n_threads, start, end, chunk);
        assert_eq!(proof, b2.proof.unwrap());
    }
}
