fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let a = &mut vec;
    a.push(11);

    let b = &mut vec;
    b.push(12);

    //In order to prevent data races, rust does not allow two mutable references pointing to the same variable at the same time.

    // I changed the order of the code. When 'a' becomes a mutable reference I call push function immediately. 
    // After that, 'a' becomes invalid and we can continue to set 'b' as mutable reference to vec.
}
