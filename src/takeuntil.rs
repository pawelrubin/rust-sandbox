use std::vec;

#[derive(Clone)]
pub struct TakeUntil<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I, P> TakeUntil<I, P> {
    pub fn new(iter: I, predicate: P) -> TakeUntil<I, P> {
        TakeUntil {
            iter,
            flag: false,
            predicate,
        }
    }
}

impl<I: Iterator, P> Iterator for TakeUntil<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            let x = self.iter.next()?;
            self.flag = (self.predicate)(&x);
            Some(x)
        }
    }
}

trait TakeUntilExtension<P>
where
    Self: Sized,
{
    fn take_until(self, predicate: P) -> TakeUntil<Self, P>;
}

impl<I, P> TakeUntilExtension<P> for I
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    fn take_until(self, predicate: P) -> TakeUntil<Self, P> {
        TakeUntil {
            iter: self,
            flag: false,
            predicate,
        }
    }
}

fn main() {
    assert_eq!(
        (0..).take_until(|&e| e >= 5).collect::<Vec<i32>>(),
        vec![0, 1, 2, 3, 4, 5]
    );
}
