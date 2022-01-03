#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstName {
    value: String,
}

impl AstName {
    pub fn new(value: String) -> Self {
        AstName { value }
    }

    pub fn eq_str(&self, value: &str) -> bool {
        self.value.eq(&String::from(value))
    }

    pub fn lt(&self, other: &Self) -> bool {
        if self.value.len() < other.value.len() {
            true
        } else if self.value.len() > other.value.len() {
            false
        } else {
            let mut result = false;

            let self_bytes = self.value.as_bytes();
            let other_bytes = other.value.as_bytes();
            for i in 0..self.value.len() {
                if self_bytes[i] < other_bytes[i] {
                    result = true;
                    break;
                } else if self_bytes[i] > other_bytes[i] {
                    result = false;
                    break;
                }
            }
            result
        }
    }

    pub fn hash(&self) -> u32 {
        let mut hash_value: u32 = 2166136261;

        for ch in self.value.chars() {
            hash_value ^= u32::from(ch);
            hash_value *= 16777619;
        }

        hash_value
    }
}
