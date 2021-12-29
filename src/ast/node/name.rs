pub struct AstName {
    value: String,
}

impl PartialEq for AstName {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl AstName {
    pub fn new(value: &str) -> Self {
        AstName {
            value: String::from(value),
        }
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
}
