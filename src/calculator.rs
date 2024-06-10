pub fn add(operands: &[f64]) -> f64 {
    operands.iter().sum()
}

pub fn subtract(operands: &[f64]) -> f64 {
    operands[0] - operands[1]
}

pub fn multiply(operands: &[f64]) -> f64 {
    operands.iter().product()
}

pub fn divide(numbers: &[f64]) -> Option<f64> {
    if numbers.len() != 2 {
        return None;
    }
    if numbers[1] == 0.0 {
        return None;
    }
    Some(numbers[0] / numbers[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(&[12.0, 24.0]), 36.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(&[13.0, 22.0]), -9.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(&[2.0, 3.0, 4.0]), 24.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(&[10.0, 2.0]), Some(5.0));
        assert_eq!(divide(&[10.0, 0.0]), None);
    }
}
