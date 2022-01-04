import java.util.ArrayList;
import java.util.Collections;

public class Main {
    public static void main(String[] args) {
        int result = largestPalindromeProduct(100, 1000);
        System.out.println("result: " + result);
    }

    static int reverseNumber(int num) {
        int reversed = 0;
        while (num > 0) {
            reversed = (reversed * 10) + (num % 10);
            num = num / 10;
        }
        return reversed;
    }

    static boolean isPalindrome(int num) {
        boolean result = num == reverseNumber(num);
        return result;
    }

    static int largestPalindromeProduct(int low, int high) {
        ArrayList<Integer> palindromes = new ArrayList<Integer>();
        for (int i = low; i < high; i++) {
            for (int j = low; j < high; j++) {
                int product = i * j;
                if (isPalindrome(product)) {
                    palindromes.add(product);
                }
            }
        }
        //palindromes.sort();
        Collections.sort(palindromes);
        return palindromes.get(palindromes.size() - 1);
    }
}
