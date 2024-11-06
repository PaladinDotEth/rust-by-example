/*
 * Activities
 * Fix the issue in the above code (see FIXME) so that it runs without error.
 * Try uncommenting the line that attempts to format the Structure struct (see TODO)
 * Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
 */

fn main() {
    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}");
}
