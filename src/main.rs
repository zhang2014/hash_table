use crate::hash_table::{DefaultHashTableEntity, DefaultHashTableGrower, HashTable};
use crate::hash_table::DefaultHasher;
use crate::hash_table::DefaultAllocator;

mod hash_table;

fn main() {
    // rust_hash_map_bench(100000000);
    // rust_btree_map_bench(100000000);
    hash_table_bench(100000000);
}

fn hash_table_bench(bench_count: i32) {
    let system_time = std::time::SystemTime::now();
    let inserted = true;
    let mut hash_table = HashTable::<i32, DefaultHashTableEntity, DefaultHasher<i32>, DefaultHashTableGrower, DefaultAllocator>::new();
    for index in 0..bench_count {
        hash_table.insert_key(index, inserted);
    }

    println!("Hash Table insert {:?} elements using {:?} milliseconds", bench_count, system_time.elapsed().unwrap().as_millis());

    for index in 0..bench_count {
        match hash_table.find_key(&index) {
            Some(entity) => unsafe {
                if entity.as_ref().unwrap().key != index {
                    println!("Error for {:?}, key: {:?}", index, entity.as_ref().unwrap().key);
                    return;
                }
            }
            None => {
                println!("Error for {:?}", index);
                return;
            }
        }
    }
}

fn rust_hash_map_bench(bench_count: i32) {
    let system_time = std::time::SystemTime::now();
    let mut hash_map = std::collections::HashMap::new();
    for index in 0..(bench_count + 1) {
        hash_map.insert(index, index);
    }

    println!("Rust Hash Map insert {:?} elements using {:?} milliseconds", bench_count, system_time.elapsed().unwrap().as_millis());
}

fn rust_btree_map_bench(bench_count: i32) {
    let system_time = std::time::SystemTime::now();
    let mut btree_map = std::collections::BTreeMap::new();
    for index in 0..bench_count {
        btree_map.insert(index, index);
    }

    println!("Rust Btree Map insert {:?} elements using {:?} milliseconds", bench_count, system_time.elapsed().unwrap().as_millis());
}
