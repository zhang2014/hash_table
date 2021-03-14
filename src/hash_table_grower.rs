pub trait IHashTableGrower {
    fn max_size(&self) -> isize;
    fn overflow(&self, size:usize) ->bool;
    fn place(&self, hash_value: u64) -> isize;
    fn next_place(&self, old_place: isize) -> isize;

    fn increase_size(&mut self);
}

#[derive(Clone)]
pub struct DefaultHashTableGrower {
    size_degree: u8
}


impl Default for DefaultHashTableGrower
{
    fn default() -> Self {
        return DefaultHashTableGrower {
            size_degree: 8
        };
    }
}

impl IHashTableGrower for DefaultHashTableGrower {
    fn max_size(&self) -> isize {
        return (1 as isize) << self.size_degree;
    }

    fn overflow(&self, size: usize) -> bool {
        return size > ((1 as usize) << (self.size_degree - 1));
    }

    fn place(&self, hash_value: u64) -> isize {
        return hash_value as isize & (((1 as isize) << self.size_degree) - 1);
    }

    fn next_place(&self, old_place: isize) -> isize {
        return (old_place + 1) & (((1 as isize) << self.size_degree) - 1);
    }

    fn increase_size(&mut self) {
        self.size_degree += if self.size_degree >= 23 { 1 } else { 2 };
    }
}
