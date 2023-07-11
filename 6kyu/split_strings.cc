/*

    https://www.codewars.com/kata/515de9ae9dcfc28eb6000001

    Instructions:

    Complete the solution so that it splits the string into pairs of two characters.
    If the string contains an odd number of characters,
    then it should replace the missing second character of the final pair with an underscore ('_').

    Examples:

    * 'abc' =>  ['ab', 'c_']
    * 'abcdef' => ['ab', 'cd', 'ef']

*/

#include <string>
#include <vector>

std::vector<std::string> solution(const std::string &s) {
  std::vector<std::string> pairs;
  for (std::size_t i = 0; i < s.length(); i += 2)
    pairs.push_back({s[i], (i + 1 < s.length() ? s[i + 1] : '_')});
  return pairs;
}
