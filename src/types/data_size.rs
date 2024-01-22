pub trait DataSizes {
    fn mb_to_bytes(self) -> Self;
    fn gb_to_bytes(self) -> Self;
}

macro_rules! impl_datasizes {
    ( $x:ty ) => {
        impl DataSizes for $x {
            fn mb_to_bytes(self) -> Self {
                self * 1_000_000
            }

            fn gb_to_bytes(self) -> Self {
                self * 1_000_000_000
            }
        }
    };
}

impl_datasizes!(usize);
impl_datasizes!(u64);
impl_datasizes!(u32);
impl_datasizes!(i64);
impl_datasizes!(i32);
