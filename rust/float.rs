#[derive(PartialEq, PartialOrd)]
struct Float<T>(T);

impl<T: PartialEq> Eq for Float<T> {}

impl<T: PartialOrd> Ord for Float<T> {
    fn cmp(&self, other: &Float<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
