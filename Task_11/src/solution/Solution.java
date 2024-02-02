package solution;

import model.IUniverse;
import model.Universe;
import model.Universe2;

import java.io.IOException;
import java.util.List;

public class Solution {

    private final IUniverse universe;

    public Solution(String inputPath) throws IOException {
        this.universe = new Universe2(InputReader.readLines(inputPath));
    }

    public long solve() {
        universe.expand();
        return universe.computeShortestPath();
    }
}
