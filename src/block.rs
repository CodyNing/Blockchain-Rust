use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
//use std::fmt::Write;
use std::sync;

type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]
pub struct Block {
    pub prev_hash: Hash,
    pub generation: u64,
    pub difficulty: u8,
    pub data: String,
    pub proof: Option<u64>,
}

impl Block {
    pub fn initial(difficulty: u8) -> Block {
        // TODO: create and return a new initial block
        Block{
            prev_hash: [0u8; 32].into(),
            generation: 0,
            difficulty,
            data: "".into(),
            proof: None
        }
    }

    pub fn next(previous: &Block, data: String) -> Block {
        // TODO: create and return a block that could follow `previous` in the chain
        Block{
            prev_hash: previous.hash(), //hash
            generation: previous.generation + 1,
            difficulty: previous.difficulty,
            data,
            proof: None
        }
    }

    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        // TODO: return the hash string this block would have if we set the proof to `proof`.
        format!(
            "{:x}:{}:{}:{}:{}",
            self.prev_hash, 
            self.generation, 
            self.difficulty, 
            self.data, 
            proof
        )
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        // TODO: return the block's hash as it would be if we set the proof to `proof`.
        let mut hasher = Sha256::new();
        let hash_str = self.hash_string_for_proof(proof);
        hasher.update(hash_str);
        hasher.finalize()
    }

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(self: &mut Block, proof: u64) {
        self.proof = Some(proof);
    }

    pub fn is_valid_for_proof(&self, proof: u64) -> bool {
        // TODO: would this block be valid if we set the proof to `proof`?
        let hash = self.hash_for_proof(proof);
        let n_bytes = usize::from(self.difficulty / 8);
        let n_bits = usize::from(self.difficulty % 8);
        for i in 0..n_bytes{
            if hash[31 - i] != 0u8 {
                return false;
            }
        }
        let next_byte = hash[31 - n_bytes];
        next_byte % (1 << n_bits) == 0
    }

    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    // Mine in a very simple way: check sequentially until a valid hash is found.
    // This doesn't *need* to be used in any way, but could be used to do some mining
    // before your .mine is complete. Results should be the same as .mine (but slower).
    pub fn mine_serial(self: &mut Block) {
        let mut p = 0u64;
        while !self.is_valid_for_proof(p) {
            p += 1;
        }
        self.proof = Some(p);
    }

    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, chunks: u64) -> u64 {
        // TODO: with `workers` threads, check proof values in the given range, breaking up
	// into `chunks` tasks in a work queue. Return the first valid proof found.
        // HINTS:
        // - Create and use a queue::WorkQueue.
        // - Use sync::Arc to wrap a clone of self for sharing.
        let mut q = WorkQueue::<MiningTask>::new(workers);
        let mut i = start;
        let shared_self = sync::Arc::<Block>::new(self.clone());
        while i < end{
            let chunk_end = if i + chunks < end {i + chunks} else {end};
            let task = MiningTask::new(shared_self.clone(), i, chunk_end);
            match q.enqueue(task){
                Ok(_) => {}
                Err(e) => {eprintln!("Mining failed: {}", e)}
            }
            i += chunks;
        }
        q.recv()
    }

    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}

struct MiningTask {
    block: sync::Arc<Block>,
    start: u64,
    end: u64
    // TODO: more fields as needed
}

impl MiningTask {
    // TODO: implement MiningTask::new(???) -> MiningTask
    pub fn new(block: sync::Arc::<Block>, start: u64, end: u64) -> MiningTask{
        MiningTask{
            block,
            start,
            end
        }
    }
}

impl Task for MiningTask {
    type Output = u64;

    fn run(&self) -> Option<u64> {
        // TODO: what does it mean to .run?
        for proof in self.start..self.end{
            if self.block.is_valid_for_proof(proof){
                return Some(proof)
            }
        }
        None
    }
}
