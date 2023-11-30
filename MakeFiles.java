import java.io.File;
import java.io.IOException;

class MakeFiles {
    public static void main(String[] args) throws IOException {
        for (int i = 1 ; i <= 25 ; i++) {
            var s = String.format("Day%02d.java", i);
            System.out.println(s);
            File f = new File(s);
            f.createNewFile();
        }
    }
}