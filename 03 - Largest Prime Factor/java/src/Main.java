public class Main {
    public static void main(String[] args) {
        long result = largestPrimeFactor(600851475143L);
        System.out.println("result: " + result);
    }

    static long largestPrimeFactor(long num) {
        long original = num;
        long factor = 3;
        long previous = 3;
        while (factor <= num) {
            if (num % factor == 0) {
                previous = factor;
                while (num % factor == 0) {
                    num /= factor;
                }
            }

            if (factor * factor > original) {
                break;
            }

            factor += 1;
        }

        return previous;
    }
}
