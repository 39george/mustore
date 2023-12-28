pub trait DataSizes {
    fn mb(self) -> Self;
    fn gb(self) -> Self;
}

impl DataSizes for u64 {
    fn mb(self) -> Self {
        self * 1_000_000
    }

    fn gb(self) -> Self {
        self * 1_000_000_000
    }
}
