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
