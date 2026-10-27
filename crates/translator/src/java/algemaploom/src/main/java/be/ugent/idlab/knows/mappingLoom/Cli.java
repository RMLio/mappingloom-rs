package be.ugent.idlab.knows.mappingLoom;
import java.io.*;

/**
 * Translates the mapping of a file, or of standard input, and prints the plan. The exit codes are those of
 * translator-bin: 1 for a mapping that can't be translated, 2 for usage and request errors, 3 for internal ones.
 */
class Cli {
    private static final int INVALID_ARGUMENT_ERROR = 2;

    public static void main(String[] args) throws IOException {
        if (args.length < 1 || args.length > 2) {
            System.err.println("Usage: java -jar MappingLoom-<VERSION>.jar <language> [<mapping>]: the language is rml-kgc, rml-io, r2rml or shexml; without a mapping file, the mapping is read from stdin");
            System.exit(INVALID_ARGUMENT_ERROR);
        }

        String mapping;
        try (Reader in = args.length == 1 ? new InputStreamReader(System.in) : new FileReader(args[1])) {
            StringWriter content = new StringWriter();
            in.transferTo(content);
            mapping = content.toString();
        } catch (IOException e) {
            System.err.println("Reading mapping failed: " + e);
            System.exit(INVALID_ARGUMENT_ERROR);
            return;
        }

        try {
            Translation translation = new Translator().translate(mapping, new Options(args[0]));
            translation.warnings().forEach(System.err::println);
            System.out.println(translation.plan());
        } catch (TranslationException e) {
            e.warnings().forEach(System.err::println);
            e.errors().forEach(System.err::println);
            System.exit(exitCode(e));
        }
    }

    static int exitCode(TranslationException e) {
        return e.errors().stream().mapToInt(error -> switch (error.kind()) {
            case "request" -> INVALID_ARGUMENT_ERROR;
            case "internal" -> 3;
            default -> 1;
        }).max().orElse(1);
    }
}
