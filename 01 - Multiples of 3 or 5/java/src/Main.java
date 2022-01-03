public class Main {
    public static void main(String[] args) {
        int result = multiplesOf3or5(1000);
        System.out.println("result: " + result);
    }

    static int multiplesOf3or5(int limit) {
        int sum = 0;
        for (int i = 1; i < limit; i++) {
            if (i % 3 == 0 || i % 5 == 0) {
                System.out.println(i);
                sum += i;
            }
        }
        return sum;
    }
}
