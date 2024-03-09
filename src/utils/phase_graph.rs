pub mod prelude { }

#[derive(Clone)]
pub struct PhaseGraphSlope<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd
{
    iterator_pre: T,
    iterator_post: T,
}

impl<T, X, Y> PhaseGraphSlope<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd
{
    pub fn wrap(iterator: T) -> Self {
        let mut iterator_post = iterator.clone();
        iterator_post.next();

        Self {
            iterator_pre: iterator,
            iterator_post,
        }
    }
}

impl<T, X, Y> Iterator for PhaseGraphSlope<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd
{
    type Item = (Y, Y);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator_pre.next()
            .zip(self.iterator_post.next())
            .map(|((_, y0),(_, y1))| (y0, y1))
    }
}

#[derive(Clone)]
pub struct PhaseGraphLines<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd + Clone
{
    iterator_pre: T,
    iterator_post: T,
    pre_last_moved: bool,
    last_coord: (Y, Y)
}

impl<T, X, Y> PhaseGraphLines<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd + Clone
{
    pub fn wrap(iterator: T) -> Self {
        let mut iterator_post = iterator.clone();
        let Some((_, y)) = iterator_post.next() else { panic!() };
        
        Self {
            iterator_pre: iterator,
            iterator_post,
            pre_last_moved: false,
            last_coord: (y.clone(), y),
        }
    }
}

impl<T, X, Y> Iterator for PhaseGraphLines<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd + Clone
{
    type Item = (Y, Y);

    fn next(&mut self) -> Option<Self::Item> {
        let (last_pre, last_post) = self.last_coord.clone();
        let pre_last_moved = self.pre_last_moved;
        self.pre_last_moved = !self.pre_last_moved;
        if pre_last_moved {
            let (_, post) = self.iterator_post.next()?;
            self.last_coord = (last_pre, post);
            Some(self.last_coord.clone())
        } else {
            let (_, pre) = self.iterator_pre.next()?;
            self.last_coord = (pre, last_post);
            Some(self.last_coord.clone())
        }
    }
}