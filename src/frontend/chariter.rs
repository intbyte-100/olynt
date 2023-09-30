
pub(crate) struct CharIterator<I: Iterator<Item = (usize, char)>> {
    iter: I,
    previous: Option<(usize, char)>,
    go_back: bool
}

impl<I> CharIterator<I> where I: Iterator<Item = (usize, char)> {
    

    pub(crate) fn new(iter: I) -> Self {
        CharIterator {iter, previous: None, go_back: false}
    }

    #[inline(always)]
    pub(crate) fn next(&mut self) -> Option<(usize, char)> {
        if self.go_back {
            self.go_back = false;
            return self.previous
        }

        
        self.previous = self.iter.next();
        self.previous
    }

    #[inline(always)]
    pub(crate) fn go_back(&mut self) {
        self.go_back = true
    }

}