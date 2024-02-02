package model;

import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class HotSpringRow {

    public final static char OPERATIONAL = '.';
    public final static char DAMAGED = '#';
    public final static char UNKNOWN = '?';

    String row;
    List<Integer> damagedSprings;

    public HotSpringRow(String row) {
        String[] split = row.split(" ");
        this.row = split[0];

        String[] split1 = split[1].split(",");
        this.damagedSprings = Arrays.stream(split1)
                .map(Integer::parseInt)
                .collect(Collectors.toList());
    }

    /**
     *          0123456789
     * example: ?.???.?#???#???.? 3,6,1
     */
    public int calculatePermutations() {
        return 0;
    }
}
