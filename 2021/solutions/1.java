import java.io.File;
import java.util.Scanner;

class Main {
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
