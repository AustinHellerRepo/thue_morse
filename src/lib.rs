
pub struct ThueMorseProvider {
    base: u32,
}

impl ThueMorseProvider {
    pub fn new(base: u32) -> Self {
        Self {
            base,
        }
    }
    #[inline(always)]
    pub fn get_value(&self, index: usize) -> u32 {
        match self.base {
            2 => {
                index.count_ones() % 2
            },
            base => {
                let mut index = index;
                let base = base as usize;
                let mut result: usize = 0;
                while index > 0 {
                    result += index % base;
                    index /= base;
                }
                (result % base) as u32
            },
        }
    }
    pub fn into_iter(self) -> ThueMorseIterator {
        ThueMorseIterator {
            thue_morse_provider: self,
            index: 0,
            is_maximum_reached: false,
        }
    }
}

pub struct ThueMorseIterator {
    thue_morse_provider: ThueMorseProvider,
    index: usize,
    is_maximum_reached: bool,
}

impl Iterator for ThueMorseIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_maximum_reached {
            return None;
        }

        let value = self.thue_morse_provider.get_value(self.index);

        // if this is the last valid value we can return, return None for any future iterations
        match self.index.checked_add(1) {
            Some(next_index) => {
                self.index = next_index;
            },
            None => {
                self.is_maximum_reached = true;
            },
        }

        Some(value)
    }
}

pub struct EvilNumberIterator {
    thue_morse_provider: ThueMorseProvider,
    index: usize,
    is_maximum_reached: bool,
}

impl EvilNumberIterator {
    pub fn new() -> Self {
        Self {
            thue_morse_provider: ThueMorseProvider::new(2),
            index: 0,
            is_maximum_reached: false,
        }
    }
}

impl Iterator for EvilNumberIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_maximum_reached {
            return None;
        }

        let mut value = self.thue_morse_provider.get_value(self.index);
        while value != 0 {
            match self.index.checked_add(1) {
                Some(next_index) => {
                    self.index = next_index;
                },
                None => {
                    self.is_maximum_reached = true;
                    return None;
                },
            }
            value = self.thue_morse_provider.get_value(self.index);
        }
        let found_at_index = self.index;
        match self.index.checked_add(1) {
            Some(next_index) => {
                self.index = next_index;
            },
            None => {
                self.is_maximum_reached = true;
            },
        }
        Some(found_at_index)
    }
}

pub struct OdiousNumberIterator {
    thue_morse_provider: ThueMorseProvider,
    index: usize,
    is_maximum_reached: bool,
}

impl OdiousNumberIterator {
    pub fn new() -> Self {
        Self {
            thue_morse_provider: ThueMorseProvider::new(2),
            index: 0,
            is_maximum_reached: false,
        }
    }
}

impl Iterator for OdiousNumberIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_maximum_reached {
            return None;
        }

        let mut value = self.thue_morse_provider.get_value(self.index);
        while value != 1 {
            match self.index.checked_add(1) {
                Some(next_index) => {
                    self.index = next_index;
                },
                None => {
                    self.is_maximum_reached = true;
                    return None;
                },
            }
            value = self.thue_morse_provider.get_value(self.index);
        }
        let found_at_index = self.index;
        match self.index.checked_add(1) {
            Some(next_index) => {
                self.index = next_index;
            },
            None => {
                self.is_maximum_reached = true;
            },
        }
        Some(found_at_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_r2n8_binary() {
        let provider = ThueMorseProvider::new(2);
        assert_eq!(0, provider.get_value(0));
        assert_eq!(1, provider.get_value(1));
        assert_eq!(1, provider.get_value(2));
        assert_eq!(0, provider.get_value(3));
        assert_eq!(1, provider.get_value(4));
        assert_eq!(0, provider.get_value(5));
        assert_eq!(0, provider.get_value(6));
        assert_eq!(1, provider.get_value(7));
    }

    #[test]
    fn test_w8m2_trinary() {
        let expected_values = [
            0u32,
            1,
            2,
            1,
            2,
            0,
            2,
            0,
            1,
            1,
            2,
            0,
            2,
            0,
            1,
            0,
            1,
            2,
            2,
            0,
            1,
            0,
            1,
            2,
            1,
            2,
            0,
        ];
        let iterator = ThueMorseProvider::new(3).into_iter();
        for (expected_value, actual_value) in expected_values.into_iter().zip(iterator) {
            println!("comparing {} to {}", actual_value, expected_value);
            assert_eq!(expected_value, actual_value);
        }
    }
    #[test]
    fn test_j1v5_five() {
        let expected_values = [0, 1, 2, 3, 4, 1, 2, 3, 4, 0, 2, 3, 4, 0, 1, 3, 4, 0, 1, 2, 4, 0, 1, 2, 3, 1, 2, 3, 4, 0];
        let iterator = ThueMorseProvider::new(5).into_iter();
        for (expected_value, actual_value) in expected_values.into_iter().zip(iterator) {
            println!("comparing {} to {}", actual_value, expected_value);
            assert_eq!(expected_value, actual_value);
        }
    }
    #[test]
    fn test_u6x9_evil_numbers() {
        let expected_values = [0, 3, 5, 6, 9, 10, 12, 15, 17, 18, 20, 23, 24, 27, 29, 30, 33, 34, 36, 39];
        let iterator = EvilNumberIterator::new();
        for (expected_value, actual_value) in expected_values.into_iter().zip(iterator) {
            println!("comparing {} to {}", actual_value, expected_value);
            assert_eq!(expected_value, actual_value);
        }
    }
    #[test]
    fn test_q5p8_odious_numbers() {
        let expected_values = [1, 2, 4, 7, 8, 11, 13, 14, 16, 19, 21, 22, 25, 26, 28, 31, 32, 35, 37, 38];
        let iterator = OdiousNumberIterator::new();
        for (expected_value, actual_value) in expected_values.into_iter().zip(iterator) {
            println!("comparing {} to {}", actual_value, expected_value);
            assert_eq!(expected_value, actual_value);
        }
    }
}