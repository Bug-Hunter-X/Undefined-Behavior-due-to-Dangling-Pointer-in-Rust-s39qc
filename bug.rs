fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    // ... some operations on the vector
    unsafe {
        // Accessing v after deallocation leads to undefined behavior
        println!("Value at index 0: {}", *ptr);
    }
}