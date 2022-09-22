pub fn sieve_of_eratosthenes(max_value: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    if max_value < 2 {
        return result;
    }

    let mut elements = vec![true; (max_value - 1).try_into().unwrap()];
    let mut current_element = 2;
    // * Round up to the nearest integer.
    let upper_limit = f64::sqrt(max_value as f64).ceil() as u64;
    while current_element <= upper_limit {
        for num in current_element..=max_value {
            let index = (num as usize) - 2;
            if num == current_element {
                elements[index] = false;
                result.push(num);
                continue;
            }
            if num % current_element == 0 {
                elements[index] = false;
            }
        }

        let next = elements.iter().position(|&element| element).unwrap();
        current_element = (next as u64) + 2;
    }

    elements.iter().enumerate().for_each(|(index, element)| {
        if *element {
            result.push((index as u64) + 2);
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn value_under_1_should_return_empty_vec() {
        assert_eq!(0, sieve_of_eratosthenes(1).len());
    }

    #[test]
    fn value_over_2_should_return_prime_numbers() {
        assert_eq!([2, 3, 5, 7, 11, 13, 17, 19, 23].to_vec(), sieve_of_eratosthenes(23));
    }
}
