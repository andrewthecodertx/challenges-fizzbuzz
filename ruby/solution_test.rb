require 'minitest/autorun'
require './solution'

class FizzBuzzTest < Minitest::Test
  def test_fizzbuzz
    result = fizzbuzz
    assert_equal 100, result.length
    assert_equal "1", result[0]
    assert_equal "Fizz", result[2]
    assert_equal "Buzz", result[4]
    assert_equal "FizzBuzz", result[14]
  end
end
