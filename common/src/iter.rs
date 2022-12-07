pub trait UnwrapSingle {
    type Item: Sized;
    fn unwrap_single(self) -> Self::Item;
    fn expect_single(self, msg: &str) -> Self::Item;
}

impl<I> UnwrapSingle for I
where
    I: IntoIterator,
{
    type Item = I::Item;

    fn unwrap_single(self) -> Self::Item {
        self.expect_single("a single item")
    }

    fn expect_single(self, msg: &str) -> Self::Item {
        let mut iter = self.into_iter();
        let item = iter.next().expect(msg);
        assert!(iter.next().is_none(), "{}", msg);
        item
    }
}
