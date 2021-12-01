import java.util.Scanner;

class Main {

	public static void main(String[] args) {
		partTwo();
	}
	
	private static void partOne() {
		try (Scanner sc = new Scanner(System.in)) {
			int last = sc.nextInt(), c = 0;
			while (sc.hasNextInt()) {
				int curr = sc.nextInt();
				if (curr > last)
					c++;
				last = curr;
			}
			System.out.println(c);
		}
	}

	private static void partTwo() {
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

