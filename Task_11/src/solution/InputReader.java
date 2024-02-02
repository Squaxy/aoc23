package solution;

import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class InputReader {

    private static final Charset ENCODING = StandardCharsets.UTF_8;

    public static List<String> readLines(String path) throws IOException {
        return Files.readAllLines(Paths.get(path), ENCODING);
    }

}
