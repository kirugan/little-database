use std::collections::HashMap;

mod block_manager;

type OID = int;

fn main() {
    let mut manager = block_manager::BlockManager::open("H:\\RustLearning\\commit.log", 4026).unwrap();
    let block = manager.read_block(0)?;

    let pool = BufferPool::new();
    page = pool.get_page(0);





    println!("done")
}