//#![feature(const_fn)]
const fn init_len() -> usize {
    return 5;
    //must be a known number
}


pub(crate) fn about() {
    let arr = [0; init_len()];
    println!("CTFE(Compile-Time Function Execution) \n \
    when a function is actually a const value, take: \n \
    const fn init_len() -> usize, and return 5 for example. \n \
    The value will be used in the correct position when the program is being compiled. \n \
    Just like the arr:[0;init_len()] -> [0;5] \n \
    the arr is :{:?}", arr);
}

