#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize

        // Can you guess what the new capacity will be?
        // Beware that the standard library makes no guarantees about the
        // algorithm used to resize the vector, so this may change in the future.
        assert_eq!(v.capacity(), 4);

        v.push(4);
        v.push(5); // beyond capacity, needs to resize
        assert_eq!(v.capacity(), 8);

        v.push(6);
        v.push(7);
        v.push(8);
        v.push(9); // beyond capacity, needs to resize
        assert_eq!(v.capacity(), 16);
    }
}
