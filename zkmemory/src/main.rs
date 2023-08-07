use zkmemory::memory::{Address256, GenericMemory, RawMemory, Word256};
// type inference lets us omit an explicit type signature (which
// would be `RBTree<&str, &str>` in this example).

fn main() {
    let mut raw_mem = RawMemory::<Address256, Word256>::new(256);

    raw_mem.write(
        Address256::from(0),
        Word256::from_limbs([0, 0, 0, 0xffaa22]),
    );

    println!("{:?}", raw_mem.read(Address256::from(0)));

    let mut raw_mem64 = RawMemory::<u64, u64>::new(64);

    raw_mem64.write(0, 1);
    raw_mem64.write(8, 2);

    println!("{:?}", raw_mem64.read(8));
}
