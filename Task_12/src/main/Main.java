package main;

import java.io.IOException;

import solution.Solution;

@SuppressWarnings("CallToPrintStackTrace")
public class Main {

    private static final String RESOURCE_ROOT = "./resources/";
    private static final String INPUT_FILE = "input.txt";

    public static void main(String[] args) {
        try {
            Solution solution = new Solution(RESOURCE_ROOT + INPUT_FILE);

            System.out.println(solution.solve());
        } catch(IOException e) {
            System.out.println("Input file couldn't be read!");
            e.printStackTrace();
        }
    }
}

