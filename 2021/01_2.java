import java.util.Scanner;

class Day01 {
	public static void main(String[] args) {
		try (Scanner sc = new Scanner(System.in)) {
			int s = 0, a = sc.nextInt(), b = sc.nextInt(), c = sc.nextInt();
			while (sc.hasNextInt()) {
				int tmp = a;
				a = b;
				b = c;
				c = sc.nextInt();

				if (tmp + a + b < a + b + c)
					s++;
			}
			System.out.println(s);
		}
	}
}
