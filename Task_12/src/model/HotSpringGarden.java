package model;

import java.util.*;

public class HotSpringGarden {

    List<HotSpringRow> rows;

    public HotSpringGarden(List<String> input) {
        this.rows = new ArrayList<>();
        for (String s : input) {
            rows.add(new HotSpringRow(s));
        }
    }

    public int calculatePermutations() {
        return rows.stream()
                .mapToInt(HotSpringRow::calculatePermutations)
                .sum();
    }
}
