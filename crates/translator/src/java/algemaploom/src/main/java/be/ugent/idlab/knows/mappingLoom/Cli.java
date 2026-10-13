package be.ugent.idlab.knows.mappingLoom;
import java.io.*;

class Cli {
    private static final int NOT_FOUND_ERROR = 1;
    private static final int INVALID_ARGUMENT_ERROR = 2;
    private static final int READING_ERROR = 3;

    public static void main(String[] args) throws IOException {
        BufferedReader in = null;
	    Translator translator = new Translator();
        String mapping = "";

        /* Parse CLI arguments */
        if (args.length == 0) {
            in = new BufferedReader(new InputStreamReader(System.in));
        } else if (args.length == 1) {
            try {
                in = new BufferedReader(new FileReader(args[0]));
            } catch (FileNotFoundException e) {
                System.err.println("File not found: " + args[0]);
                System.exit(NOT_FOUND_ERROR);
            }
        } else {
            System.err.println("Usage: java -jar algemaploom-<VERSION>-jar-with-dependencies.jar <mapping> or pipe mapping via stdin");
            System.exit(INVALID_ARGUMENT_ERROR);
        }

        /* Translate */
        try {
            StringWriter content = new StringWriter();
            in.transferTo(content);
            mapping = content.toString();
        } catch (IOException e) {
            System.err.println("Reading mapping failed: " + e);
            System.exit(READING_ERROR);
        } finally {
            in.close();
        }

        String translation = translator.translate_to_document(mapping);
        System.out.println(translation);
    }
}
