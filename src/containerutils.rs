

pub fn extract_two_elements<T>(
    container: &mut [T],
    index0: usize,
    index1: usize,
) -> Option<(&mut T, &mut T)> {
    let container_len = container.len();
    let mut iter = container.iter_mut();

    if index0 < container_len && index1 < container_len {
        if index0 < index1 {
            Some((
                iter.nth(index0).unwrap(),
                iter.nth(index1 - index0 - 1).unwrap(),
            ))
        } else if index1 < index0 {
            let item1 = iter.nth(index1).unwrap();
            let item0 = iter.nth(index0 - index1 - 1).unwrap();
            Some((item0, item1))
        } else {
            None
        }
    } else {
        None
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extract_two_elements_lessthan() {
        let mut arr = [0, 1, 2, 3, 4];
        if let Some((e0, e1)) = extract_two_elements(&mut arr, 1, 3) {
            *e0 = 6;
            *e1 = 8;
        }

        assert_eq!(arr, [0, 6, 2, 8, 4]);
    }

    #[test]
    fn test_extract_two_elements_greaterthan() {
        let mut arr = [0, 1, 2, 3, 4];
        if let Some((e0, e1)) = extract_two_elements(&mut arr, 3, 1) {
            *e0 = 6;
            *e1 = 8;
        }

        assert_eq!(arr, [0, 8, 2, 6, 4]);
    }

    #[test]
    fn test_extract_two_elements_equal() {
        let mut arr = [0, 1, 2, 3, 4];
        if let Some((e0, e1)) = extract_two_elements(&mut arr, 1, 1) {
            *e0 = 6;
            *e1 = 8;
        }

        assert_eq!(arr, [0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_extract_two_elements_atend() {
        let mut arr = [0, 1, 2, 3, 4];
        if let Some((e0, e1)) = extract_two_elements(&mut arr, 1, 5) {
            *e0 = 6;
            *e1 = 8;
        }

        assert_eq!(arr, [0, 1, 2, 3, 4]);
    }
}
