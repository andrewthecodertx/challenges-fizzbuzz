import org.junit.Test;
import static org.junit.Assert.*;

public class SolutionTest {
    @Test
    public void testFizzBuzz() {
        java.util.List<String> result = Solution.fizzbuzz();
        
        assertEquals(100, result.size());
        assertEquals("1", result.get(0));
        assertEquals("Fizz", result.get(2));
        assertEquals("Buzz", result.get(4));
        assertEquals("FizzBuzz", result.get(14));
    }
}
