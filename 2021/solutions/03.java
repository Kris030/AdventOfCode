import java.util.Scanner;

class Day03 {

	public static void main(String[] args) {
		partOne();
	}
	
	public static void partOne() {
		try (Scanner sc = new Scanner(System.in)) {
			String s = sc.next();
			int bs = s.lenght();
			int[][] bc = new int[bs][2];
			
			do {
				for (int i = 0; i < bs; i++)
					bc[i][(int) (s.charAt(i) - '0')]++;
				s = sc.next();
			} while (sc.hasNext());

			int gamma = 0, epsilon = 0;
			for (int i = 0; i < bs; i++) {
				int maxi, mini;
				if (bc[i][0] > bc[i][1]) {
					maxi = 0;
					mini = 1;
				} else {
					maxi = 1;
					mini = 0;
				}
				gamma   |= maxi <<< i;
				epsilon |= mini <<< i;
			}

			System.out.println(gamma * epsilon);
		}
	}

}
