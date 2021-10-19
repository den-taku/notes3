pub fn stupid_fibonacci(index: usize) -> u64 {
    if index == 0 || index == 1 {
        1
    } else {
        stupid_fibonacci(index - 1) + stupid_fibonacci(index - 2)
    }
}
