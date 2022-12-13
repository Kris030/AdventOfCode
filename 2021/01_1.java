import java.util.Scanner;

class Day01 {

	public static void main(String[] args) {
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
}

