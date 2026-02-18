const fizzbuzz = require('./solution');

describe('FizzBuzz', () => {
  test('prints 1-100 with Fizz/Buzz replacements', () => {
    const consoleSpy = jest.spyOn(console, 'log').mockImplementation();
    fizzbuzz();
    
    const output = consoleSpy.mock.calls.map(c => c[0]);
    
    expect(output[0]).toBe(1);
    expect(output[2]).toBe('Fizz');
    expect(output[4]).toBe('Buzz');
    expect(output[14]).toBe('FizzBuzz');
    expect(output.length).toBe(100);
    
    consoleSpy.mockRestore();
  });
});
