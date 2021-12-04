import java.util.ArrayList;
import java.util.Scanner;

class Day03 {

	public static void main(String[] args) {
		partTwo();
	}

	public static void partOne() {
		try (Scanner sc = new Scanner(System.in)) {
			String s = sc.next();
			int bs = s.length();
			int[][] bc = new int[bs][2];

			for (int i = 0; i < bs; i++)
				bc[i][(int) (s.charAt(i) - '0')]++;

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

	public static void partTwo() {
		try (Scanner sc = new Scanner(System.in)) {
			ArrayList<String> mc = new ArrayList<String>();	
			ArrayList<String> lc = new ArrayList<String>();
	
			String s = sc.next();
			int bs = s.length();
	
			lc.add(s); mc.add(s);
	
			int[][] bc = new int[bs][2];
	
			for (int i = 0; i < bs; i++)
				bc[i][(int) (s.charAt(i) - '0')]++;
	
			while (sc.hasNext()) {
				s = sc.next();
				lc.add(s); mc.add(s);
				for (int i = 0; i < bs; i++)
					bc[i][(int) (s.charAt(i) - '0')]++;
			}
	
			int o = 0;
			l:
			for (int i = 0; i < bs; i++) {
	
				int ones = 0, zeros = 0;
				for (String ss : mc)
					if (ss.charAt(i) == '1')
						ones++;
					else
						zeros++;
	
				char d = zeros <= ones ? '1' : '0';
				for (int j = 0; j < mc.size();) {
					if (mc.get(j).charAt(i) == d) {
						j++; continue;
					}
	
					mc.remove(j);
					if (mc.size() == 1) {
						o = Integer.parseInt(mc.get(0), 2);
						break l;
					}
				}
			}
	
			int c = 0;
			l:
			for (int i = 0; i < bs; i++) {
	
				int ones = 0, zeros = 0;
				for (String ss : lc)
					if (ss.charAt(i) == '1')
						ones++;
					else
						zeros++;
	
				char d = zeros > ones ? '1' : '0';
				for (int j = 0; j < lc.size();) {
					if (lc.get(j).charAt(i) == d) {
						j++; continue;
					}
	
					lc.remove(j);
					if (lc.size() == 1) {
						c = Integer.parseInt(lc.get(0), 2);
						break l;
					}
				}
			}
	
			System.out.println(o * c);
		}
	}
}
