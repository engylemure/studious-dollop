// Recursive approach to solve the factorial algorithm.
mod factorial {
    pub fn factorial(value: u32) -> Option<u128> {
        internal_factorial(value as u128)
    }

    fn internal_factorial(value: u128) -> Option<u128> {
        if value > 1 {
            match internal_factorial(value - 1) {
                Some(next_value) => value.checked_mul(next_value),
                None => None
            }
        }  else {
            Some(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;


    #[test]
    fn three() {
        assert_eq!(factorial::factorial(3), Some(6));
    }

    #[test]
    fn four() {
        assert_eq!(factorial::factorial(4), Some(24));
    }

    #[test]
    fn five() {
        assert_eq!(factorial::factorial(5), Some(120));
    }

    #[test]
    fn six() {
        assert_eq!(factorial::factorial(6), Some(720));
    }

    #[test]
    fn seven() {
        assert_eq!(factorial::factorial(7), Some(5040));
    }

    #[test]
    fn eight() {
        assert_eq!(factorial::factorial(8), Some(40320));
    }
}
