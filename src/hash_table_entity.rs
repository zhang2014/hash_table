pub trait IHashTableEntity<Key>
{
    fn new_zero() -> Self;
    fn is_zero_s(key: &Key) -> bool;

    fn is_zero(&self) -> bool;
    fn set_zero(&mut self) -> ();
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
        return self.key == other.key;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl IHashTableEntity<i32> for DefaultHashTableEntity
{
    fn new_zero() -> Self {
        return DefaultHashTableEntity {
            key: 0,
            hash: 0,
        };
    }

    fn is_zero_s(key: &i32) -> bool {
        return *key == 0;
    }

    fn is_zero(&self) -> bool {
        return self.key == 0;
    }

    fn set_zero(&mut self) -> () {
        self.key = 0;
        self.hash = 0;
    }

    fn key_equals(&self, key: &i32, hash: u64) -> bool {
        return self.key == *key;
    }

    fn set_key_and_hash(&mut self, key: &i32, hash: u64) {
        self.key = *key;
        self.hash = hash;
    }

    fn get_key(&self) -> &i32 {
        return &self.key;
    }

    fn get_hash(&self) -> u64 {
        return self.hash;
    }
}

