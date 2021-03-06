pub trait Sorter<T> {
    fn sort(slice: &mut [T])
    where
        T: Ord + Copy;
}

mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;
mod mergesort;

struct StdSorter;

impl<T> Sorter<T> for StdSorter {
    fn sort(slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stdsort_works() {
        let mut slice = vec![0, 8, 5, 1, 9];
        StdSorter::sort(&mut slice);
        assert_eq!(slice, vec![0, 1, 5, 8, 9]);
    }
}
