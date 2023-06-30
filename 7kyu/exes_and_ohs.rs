/*

   https://www.codewars.com/kata/55908aad6620c066bc00002a

   Instructions:
   Check to see if a string has the same amount of 'x's and 'o's.
   The method must return a boolean and be case insensitive.
   The string can contain any char.

*/

fn xo(string: &'static str) -> bool {
    string
        .chars()
        .fold(0, |balance, c| match c.to_ascii_lowercase() {
            'x' => balance + 1,
            'o' => balance - 1,
            _ => balance,
        })
        == 0
}
