import java.util.Scanner;

class Day02 {

	public static void main(String[] args) {
		try (Scanner sc = new Scanner(System.in)) {
			int hpos = 0, depth = 0, aim = 0;
			
			while (sc.hasNextLine()) {
				String dir = sc.next();
				int x = sc.nextInt();
				
				switch (dir.charAt(0)) {
					case 'f':
						hpos += x;
						depth += aim * x;
						break;
					
					case 'u': aim -= x; break;
					case 'd': aim += x; break;
				}
			}

			System.out.println(hpos * depth);
		}
	}
}
