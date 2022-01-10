use extend::ext;

#[ext(name=IteratorUtil)]
pub impl<T:Iterator> T {
    fn drop_first(mut self) -> Self {
        self.next();
        self
    }
}

#[ext(name=DoubleEndedIteratorUtil)]
pub impl<T:DoubleEndedIterator> T {
    fn drop_last(mut self) -> Self {
        self.next_back();
        self
    }
}

