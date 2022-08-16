/*

    https://www.codewars.com/kata/5467e4d82edf8bbf40000155

    Instructions:
    Your task is to make a function that can take any non-negative integer as an argument and return it with its digits in descending order.
    Essentially, rearrange the digits to create the highest possible number.

    fn descending_order(x: u64) -> u64 {
        unimplemented!()
    }

 */

fn descending_order(x: u64) -> u64 {
    let mut c = x.to_string().chars().collect::<Vec<char>>(); // Collect all the characters from the string.
    c.sort(); // Sort the chars inside the Vector.
    c.reverse(); // Reverse the chars index in the Vector.
    String::from_iter(c).parse::<u64>().unwrap() // Convert the chars back to u64.
}