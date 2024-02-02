package model;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Universe2 implements IUniverse {

    private final static Character GALAXY_CHAR = '#';
    private final static Character EMPTY_CHAR = '.';

    private final static long EXPANSION_FACTOR = 1_000_000;

    List<List<Character>> rows;
    List<int[]> galaxies;

    Set<Integer> noGalaxyRowIdx;
    Set<Integer> noGalaxyColIdx;
    boolean expanded;


    public Universe2(List<String> input) {
        this.rows = new ArrayList<>();
        for (String s: input) {
            this.rows.add(s.chars().mapToObj(e -> (char) e).collect(Collectors.toList()));
        }

        this.expanded = false;
    }

    public void expand() {
        if (!expanded) {
            findRowIdxToDuplicate();
            findColIdxToDuplicate();
            accumulateGalaxyIndicies();
            this.expanded = true;
        }
    }

    private void accumulateGalaxyIndicies() {
        this.galaxies = new ArrayList<>();
        for (int i = 0; i < rows.size(); i++) {
            for (int j = 0; j < rows.get(i).size(); j++) {
                if (rows.get(i).get(j).equals(GALAXY_CHAR)) {
                    this.galaxies.add(new int[]{i, j});
                }
            }
        }
    }

    private void findRowIdxToDuplicate() {
        this.noGalaxyRowIdx = new HashSet<>();

        for (int i = 0; i < rows.size(); i++) {
            if (rows.get(i).get(0).equals(EMPTY_CHAR)) {
                noGalaxyRowIdx.add(i);
            }
        }

        for (int i = 0; i < rows.size(); i++) {
            for (int j = 0; j < rows.get(0).size(); j++) {
                if (rows.get(i).get(j).equals(GALAXY_CHAR)) {
                    noGalaxyRowIdx.remove(i);
                }
            }
        }
    }

    private void findColIdxToDuplicate() {
        this.noGalaxyColIdx = new HashSet<>();
        for (int i = 0; i < rows.size(); i++) {
            if (i == 0) {
                for (int j = 0; j < rows.get(0).size(); j++) {
                    if (rows.get(0).get(j).equals(EMPTY_CHAR)) {
                        noGalaxyColIdx.add(j);
                    }
                }
            } else {
                for (int j = 0; j < rows.get(i).size(); j++) {
                    if (rows.get(i).get(j).equals(GALAXY_CHAR)) {
                        noGalaxyColIdx.remove(j);
                    }
                }
            }
        }
    }

    private void duplicateRows() {
        findRowIdxToDuplicate();
        int drift = 0;
        for (int i : noGalaxyRowIdx) {
            rows.add(i + drift, new ArrayList<>(rows.get(i + drift)));
            drift++;
        }
    }

    private void duplicateCols() {
        findColIdxToDuplicate();
        int drift = 0;
        for (int i : noGalaxyColIdx) {
            for (List<Character> row : rows) {
                row.add(i + drift, EMPTY_CHAR);
            }
            drift++;
        }
    }

    public long computeShortestPath() {
        long sumOfPaths = 0;
        for (int i = 0; i < galaxies.size() - 1; i++) {
            for (int j = i + 1; j < galaxies.size(); j++) {
                sumOfPaths += calculateDistance(galaxies.get(i), galaxies.get(j));
            }
        }
        return sumOfPaths;
    }

    private long calculateDistance(int[] galaxy1, int[] galaxy2) {
        int row_min = Math.min(galaxy1[0], galaxy2[0]);
        int row_max = Math.max(galaxy1[0], galaxy2[0]);
        int col_min = Math.min(galaxy1[1], galaxy2[1]);
        int col_max = Math.max(galaxy1[1], galaxy2[1]);

        int countOfFactorApplications = 0;

        for (Integer galaxyRowIdx : noGalaxyRowIdx) {
            if (row_min < galaxyRowIdx && galaxyRowIdx < row_max) {
                countOfFactorApplications++;
            }
        }
        for (Integer galaxyColIdx : noGalaxyColIdx) {
            if (col_min < galaxyColIdx && galaxyColIdx < col_max) {
                countOfFactorApplications++;
            }
        }

        int distanceY, distanceX;

        distanceY = Math.abs(galaxy1[0] - galaxy2[0]);
        distanceX = Math.abs(galaxy1[1] - galaxy2[1]);

        return (distanceX + distanceY + ((EXPANSION_FACTOR - 1) * countOfFactorApplications));
    }
}
