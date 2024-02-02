package solution;

import model.Universe;

import java.io.IOException;
import java.util.List;

public class Solution {

    private final Universe universe;

    public Solution(String inputPath) throws IOException {
        this.universe = new Universe(InputReader.readLines(inputPath));
    }

    public long solve() {
        universe.expand();
        return universe.computeShortestPath();
    }
}
