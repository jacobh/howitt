use std::iter::Scan;

pub trait ResultIterExt<T, E> {
    fn collect_result_vec(self) -> Result<Vec<T>, E>;
}

impl<I, T, E> ResultIterExt<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
{
    fn collect_result_vec(self) -> Result<Vec<T>, E> {
        self.collect()
    }
}

pub trait ScanAllExt: Iterator {
    fn scan_all<St, B, F>(
        self,
        initial_state: St,
        mut f: F,
    ) -> Scan<Self, St, impl FnMut(&mut St, Self::Item) -> Option<B>>
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> B,
    {
        self.scan(initial_state, move |state, item| Some(f(state, item)))
    }
}

// Implement for all iterators
impl<T: Iterator> ScanAllExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_result_vec_all_ok() {
        let iter = vec![Ok(1), Ok(2), Ok(3)].into_iter();
        let result: Result<Vec<i32>, &str> = iter.collect_result_vec();
        assert_eq!(result, Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_collect_result_vec_with_error() {
        let iter = vec![Ok(1), Err("error"), Ok(3)].into_iter();
        let result: Result<Vec<i32>, &str> = iter.collect_result_vec();
        assert_eq!(result, Err("error"));
    }

    #[test]
    fn test_collect_result_vec_empty() {
        let iter: std::iter::Empty<Result<i32, &str>> = std::iter::empty();
        let result: Result<Vec<i32>, &str> = iter.collect_result_vec();
        assert_eq!(result, Ok(vec![]));
    }

    #[test]
    fn test_collect_result_vec_with_strings() {
        let iter = vec![Ok("hello".to_string()), Ok("world".to_string())].into_iter();
        let result: Result<Vec<String>, &str> = iter.collect_result_vec();
        assert_eq!(result, Ok(vec!["hello".to_string(), "world".to_string()]));
    }

    #[test]
    fn test_collect_result_vec_multiple_errors() {
        let iter = vec![Ok(1), Err("first error"), Ok(3), Err("second error"), Ok(5)].into_iter();
        let result: Result<Vec<i32>, &str> = iter.collect_result_vec();
        assert_eq!(result, Err("first error"));
    }

    #[test]
    fn test_scan_all_running_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = numbers
            .into_iter()
            .scan_all(0, |sum, x| {
                *sum += x;
                *sum
            })
            .collect();
        assert_eq!(result, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_scan_all_with_strings() {
        let words = vec!["Hello", "World", "!"];
        let result: Vec<String> = words
            .into_iter()
            .scan_all(String::new(), |acc, x| {
                if !acc.is_empty() {
                    acc.push(' ');
                }
                acc.push_str(x);
                acc.clone()
            })
            .collect();
        assert_eq!(
            result,
            vec![
                "Hello".to_string(),
                "Hello World".to_string(),
                "Hello World !".to_string()
            ]
        );
    }

    #[test]
    fn test_scan_all_empty_iterator() {
        let empty: Vec<i32> = vec![];
        let result: Vec<i32> = empty
            .into_iter()
            .scan_all(0, |sum, x| {
                *sum += x;
                *sum
            })
            .collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_scan_all_state_transformation() {
        let numbers = vec![1, 2, 3];
        let result: Vec<(i32, i32)> = numbers
            .into_iter()
            .scan_all((0, 1), |state, x| {
                state.0 += x; // sum
                state.1 *= x; // product
                *state
            })
            .collect();
        assert_eq!(result, vec![(1, 1), (3, 2), (6, 6)]);
    }
}
