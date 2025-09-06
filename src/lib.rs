/// Provides a `foldr` method (right fold) for all iterators.
///
/// Right fold applies the function from the right:
///
/// # Examples
///
/// ```ignore
/// let v = vec![1, 2, 3];
/// let result = v.into_iter().foldr(0, |x, acc| x + acc);
/// assert_eq!(result, 6);
///
/// let result2 = v.into_iter().foldr(0, |x, acc| x - acc);
/// assert_eq!(result2, 2); // 1 - (2 - (3 - 0))
/// ```
pub trait FoldR: Iterator + Sized {
    fn foldr<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(Self::Item, B) -> B;
}

impl<I: Iterator> FoldR for I {
    fn foldr<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(Self::Item, B) -> B,
    {
        let items: Vec<_> = self.collect();

        items
            .into_iter()
            .rev()
            .fold(init, |acc, num| f(num, acc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let v = vec![1, 2, 3];
        let result = v.iter().foldr(0, |x, acc| x + acc);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_subtract() {
        let v = vec![1, 2, 3];
        let result = v.iter().foldr(0, |x, acc| x - acc);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_empty() {
        let v: Vec<i32> = vec![];
        let result = v.iter().foldr(10, |x, acc| x + acc);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_strings() {
        let v = vec!["a", "b", "c"];
        let result = v.iter().foldr(String::new(), |x, acc| x.to_string() + &acc);
        assert_eq!(result, "abc");
    }
}
