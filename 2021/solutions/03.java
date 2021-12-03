import java.util.Scanner;

class Day03 {

	public static void main(String[] args) {
		partOne();
	}

	public static void partOne() {
		try (Scanner sc = new Scanner(System.in)) {
			String s = sc.next();
			int bs = s.length();
			int[][] bc = new int[bs][2];

			for (int i = 0; i < bs; i++)
				bc[i][(int) (s.charAt(i) - '0')]++

			while (sc.hasNext()) {
				s = sc.next();
				for (int i = 0; i < bs; i++)
					bc[i][(int) (s.charAt(i) - '0')]++;
			}

			int g = 0, e = 0;
			for (int i = 0; i < bs; i++) 
				if (bc[i][0] > bc[i][1])
					e |= 1 << (bs - i - 1);
				else
					g |= 1 << (bs - i - 1);

			System.out.println(g * e);
		}
	}
}
