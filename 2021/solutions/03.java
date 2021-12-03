import java.util.Scanner;

class Day03 {

	public static void main(String[] args) {
		partOne();
	}
	
	public static void partOne() {
		try (Scanner sc = new Scanner(System.in)) {
			int[][] bc = new int[5][2];
			
			while (sc.hasNext()) {
				String s = sc.next();
				for (int i = 0; i < 5; i++)
					bc[i][(int) (s.charAt(i) - '0')]++;
			}
			int gamma = 0, epsilon = 0;
			for (int i = 0; i < 5; i++) {
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
