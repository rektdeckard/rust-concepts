#[derive(Debug)]
pub struct Vector {
    data: String,
}

impl Vector {
    pub fn new(data: String) -> Vector {
        Vector { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_formattable() {
        let v = Vector::new("Test".to_string());
        assert_eq!("Test", v.data);
    }
}

