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
			int c = 0, m[] = new int[] { sc.nextInt(), sc.nextInt(), sc.nextInt() };
			while (sc.hasNextInt()) {
				int tmp = m[0];
				m[0] = m[1];
				m[1] = m[2];
				m[2] = sc.nextInt();
				
				

				if (tmp + m[0] + m[1] < m[0] + m[1] + m[2])
					c++;
			}
			System.out.println(c);
		}
	}
}

