pub trait IHashTableEntity<Key>: Sized + PartialEq
{
    fn is_zero_key(key: &Key) -> bool;
    fn key_equals(&self, key: &Key, hash: u64) -> bool;

    fn is_zero_entity(entity: &Self) -> bool;
    fn is_same_or_empty_entity(entity: &Self, key: &Key) -> bool;
    fn set_entity_key_and_hash(entity: &mut Self, key: &Key, hash: u64);

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
    #[inline(always)]
    fn is_zero_key(key: &i32) -> bool {
        *key == 0
    }

    fn key_equals(&self, key: &i32, _hash: u64) -> bool {
        self.key == *key
    }

    #[inline(always)]
    fn is_zero_entity(entity: &Self) -> bool {
        entity.key == 0
    }

    fn is_same_or_empty_entity(entity: &Self, key: &i32) -> bool {
        DefaultHashTableEntity::is_zero_entity(entity) || entity.key == key
    }

    #[inline(always)]
    fn set_entity_key_and_hash(entity: &mut Self, key: &i32, hash: u64) {
        entity.key = *key;
        entity.hash = hash;
    }

    fn get_key(&self) -> &i32 {
        unsafe { &self.key }
    }

    fn get_hash(&self) -> u64 {
        self.hash
    }
}

