fn main() {}

/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```
/// let result = doctests_exercise::div(...);  // TODO: finish this test!
/// assert_eq!(result, 2);
/// ```
///
/// # Example #2: 6 / 3 = 2
///
/// ```
/// TODO: Write this doctest!
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// TODO: Write this doctest!
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```
/// TODO: Write this doctest!
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```
/// TODO: Write this doctest!
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}