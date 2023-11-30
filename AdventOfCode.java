import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

public abstract class AdventOfCode {
    String puzzle, example;

    public AdventOfCode(String puzzle, String example) throws IOException {
        this.puzzle = Files.readString(new File(puzzle).toPath());
        this.example = Files.readString(new File(example).toPath());
    }

    abstract String part1(String input);
    abstract String part2(String input);
    
    public void solve() {
        solve1();
        solve2();
    }

    public void solve1() {
        System.out.println("Part 1: " + part1(puzzle));
    }

    public void solve2() {
        System.out.println("Part 2: " + part2(puzzle));
    }

    public void test() {
        test1();
        test2();
    }

    public void test1() {
        System.out.println("Test 1: " + part1(example));
    }

    public void test2() {
        System.out.println("Test 2: " + part2(example));
    }
}
