fn main() {
    let mut v = vec![1, 2, 3];
    { // Creating a new scope to ensure that the vector is dropped and memory is released.
        let ptr = v.as_ptr();
        unsafe {
            // Accessing elements of the vector before it goes out of scope 
            println!("Value at index 0: {}", *ptr);
            println!("Value at index 1: {}", *ptr.add(1)); 
            println!("Value at index 2: {}", *ptr.add(2));           
        }
    } 
    // v is deallocated here automatically since it goes out of scope.
    // Trying to access v after this line is undefined behavior.
}
