pub trait IHashTableEntity<Key>: Sized + PartialEq
{
    fn is_zero_key(key: &Key) -> bool;

    fn is_zero(&self) -> bool;
    fn key_equals(&self, key: &Key, hash: u64) -> bool;
    fn set_key_and_hash(&mut self, key: &Key, hash: u64);

    fn get_key(&self) -> &Key;
    fn get_hash(&self) -> u64;
}

#[repr(C, packed)]
pub struct DefaultHashTableEntity {
    pub(crate) key: i32,
    pub(crate) hash: u64,
}

impl PartialEq for DefaultHashTableEntity {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl IHashTableEntity<i32> for DefaultHashTableEntity
{
    fn is_zero_key(key: &i32) -> bool {
        *key == 0
    }

    fn is_zero(&self) -> bool {
        self.key == 0
    }

    fn key_equals(&self, key: &i32, _hash: u64) -> bool {
        self.key == *key
    }

    fn set_key_and_hash(&mut self, key: &i32, hash: u64) {
        self.key = *key;
        self.hash = hash;
    }

    fn get_key(&self) -> &i32 {
        unsafe { &self.key }
    }

    fn get_hash(&self) -> u64 {
        self.hash
    }
}

