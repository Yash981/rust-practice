// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.

fn main() {
    // You can optionally experiment here.
    let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Create an iterator over the array.
        let mut fav_fruits_iterator = my_fav_fruits.iter();
        for _i in 0..10  {
            if let Some(fruit) = fav_fruits_iterator.next() {
                println!("{:?} hello", fav_fruits_iterator.next());
            } else {
                println!("No more fruits!");
            }
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Create an iterator over the array.
        let mut fav_fruits_iterator = my_fav_fruits.iter();
        for _i in 0..10  {
            if let Some(fruit) = fav_fruits_iterator.next() {
                println!("{:?} hello", fruit);
            } else {
                println!("No more fruits!");
            }
        }

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"dragonfruit")); // TODO: Replace `todo!()`
    }
}
