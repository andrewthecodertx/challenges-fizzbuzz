using System;
using Xunit;

public class SolutionTests {
    [Fact]
    public void TestFizzBuzz() {
        var result = Solution.FizzBuzz();
        
        Assert.Equal(100, result.Count);
        Assert.Equal("1", result[0]);
        Assert.Equal("Fizz", result[2]);
        Assert.Equal("Buzz", result[4]);
        Assert.Equal("FizzBuzz", result[14]);
    }
}
