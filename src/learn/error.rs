pub fn error() {
    // panic!("Crash and Burn");
    let some_vector = vec![1, 2, 3];
    some_vector[99];
    // Use `set RUST_BACKTRACE=1` on cmd
    // or `$Env:RUST_BACKTRACE=1` on powershell
    // to backtrace error
}
