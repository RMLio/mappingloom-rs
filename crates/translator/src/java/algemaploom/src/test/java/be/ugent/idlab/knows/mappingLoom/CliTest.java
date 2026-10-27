package be.ugent.idlab.knows.mappingLoom;

import static org.junit.Assert.assertTrue;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;

import org.junit.Test;

public class CliTest
{
    // A Turtle comment ends at the newline, so the mapping after it must survive reading from stdin.
    private static final String MAPPING_WITH_COMMENT = """
            # Students to FOAF
            @prefix rr: <http://www.w3.org/ns/r2rml#> .
            @prefix foaf: <http://xmlns.com/foaf/0.1/> .
            @prefix rml: <http://semweb.mmlab.be/ns/rml#> .
            @prefix ql: <http://semweb.mmlab.be/ns/ql#> .

            <http://example.com/base/TriplesMap1> a rr:TriplesMap ;
              rml:logicalSource [ rml:source "student.csv" ; rml:referenceFormulation ql:CSV ] ;
              rr:subjectMap [ rr:template "http://example.com/{Name}" ] ;
              rr:predicateObjectMap [ rr:predicate foaf:name ; rr:objectMap [ rml:reference "Name" ] ] .
            """;

    @Test
    public void testStdinKeepsLineBreaks() throws Exception
    {
        InputStream stdin = System.in;
        PrintStream stdout = System.out;
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        try {
            System.setIn(new ByteArrayInputStream(MAPPING_WITH_COMMENT.getBytes(StandardCharsets.UTF_8)));
            System.setOut(new PrintStream(out, true, StandardCharsets.UTF_8));
            Cli.main(new String[] {"rml-io"});
        } finally {
            System.setIn(stdin);
            System.setOut(stdout);
        }

        String plan = out.toString(StandardCharsets.UTF_8);
        assertTrue("no plan on stdout: " + plan, plan.contains("Serialize"));
    }
}
