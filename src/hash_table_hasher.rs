pub trait IHasher<Key>
{
    // #[inline(always)]
    fn hash(key: &Key) -> u64;
}

pub struct DefaultHasher {}

impl IHasher<i32> for DefaultHasher
{
    #[inline(always)]
    fn hash(key: &i32) -> u64 {
        (key ^ 0xFFFFFFF_i32) as u64
        // return key as usize;
    }
}
