import { fizzbuzz } from './solution';

describe('FizzBuzz', () => {
  test('returns array of 100 elements with correct values', () => {
    const result = fizzbuzz();
    
    expect(result.length).toBe(100);
    expect(result[0]).toBe("1");
    expect(result[2]).toBe("Fizz");
    expect(result[4]).toBe("Buzz");
    expect(result[14]).toBe("FizzBuzz");
  });
});
