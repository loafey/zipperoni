pub struct Zipper<'l, T> {
    inner: &'l Vec<T>,
    index: usize,
}
impl<'l, T> From<&'l Vec<T>> for Zipper<'l, T> {
    fn from(value: &'l Vec<T>) -> Self {
        Self {
            inner: value,
            index: 0,
        }
    }
}
impl<'l, T> Iterator for Zipper<'l, T> {
    type Item = (&'l [T], &'l T, &'l [T]);

    fn next(&mut self) -> Option<Self::Item> {
        let middle = self.inner.get(self.index)?;
        let left = &self.inner[(0..self.index)];
        let right = &self.inner[self.index + 1..self.inner.len()];
        self.index += 1;
        Some((left, middle, right))
    }
}

pub struct Zipper2D<'l, T> {
    inner: &'l Vec<Vec<T>>,
    index_x: usize,
    index_y: usize,
}
impl<'l, T> From<&'l Vec<Vec<T>>> for Zipper2D<'l, T> {
    fn from(value: &'l Vec<Vec<T>>) -> Self {
        Self {
            inner: value,
            index_x: 0,
            index_y: 0,
        }
    }
}
impl<'l, T> Iterator for Zipper2D<'l, T> {
    // Could this be nicely done without allocating vecs? 🤔
    // Maybe one could use sized arrays instead, but those are
    // still not continuously allocated in the stack which the
    // vertical slices would require.
    type Item = (&'l [T], &'l [T], &'l T, Vec<&'l T>, Vec<&'l T>);

    fn next(&mut self) -> Option<Self::Item> {
        let middle = self.inner.get(self.index_y)?.get(self.index_x)?;

        let left = &self.inner.get(self.index_y)?[0..self.index_x];
        let right =
            &self.inner.get(self.index_y)?[self.index_x + 1..self.inner.get(self.index_y)?.len()];

        let up = (self.inner[0..self.index_y].iter().map(|v| &v[self.index_x])).collect();
        let down = (self.inner[self.index_y + 1..self.inner.len()]
            .iter()
            .map(|v| &v[self.index_x]))
        .collect();

        self.index_x += 1;
        if self.index_x >= self.inner.get(self.index_y)?.len() {
            self.index_x = 0;
            self.index_y += 1;
        }
        Some((left, right, middle, up, down))
    }
}
