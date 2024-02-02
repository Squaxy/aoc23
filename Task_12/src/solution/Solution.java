package solution;

import model.HotSpringGarden;

import java.io.IOException;

public class Solution {

    private final HotSpringGarden garden;

    public Solution(String inputPath) throws IOException {
        this.garden = new HotSpringGarden(InputReader.readLines(inputPath));
    }

    public int solve() {
        return garden.calculatePermutations();
    }
}
