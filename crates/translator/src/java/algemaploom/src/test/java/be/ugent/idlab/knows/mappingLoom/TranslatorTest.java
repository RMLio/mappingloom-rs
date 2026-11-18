package be.ugent.idlab.knows.mappingLoom;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertThrows;
import static org.junit.Assert.assertTrue;

import java.util.List;

import org.junit.Test;

/**
 * Unit test for library API
 */
public class TranslatorTest
{
    private static String SHEXML_EXAMPLE = """
            PREFIX : <http://example.com/>
            PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
            SOURCE films_csv_file <./film.csv>
            SOURCE films_second_csv_file <./film2.csv>

            ITERATOR film_csv <csvperrow> {
                FIELD id <@id>
                FIELD name <name>
                FIELD year <year>
                FIELD country <country>
                FIELD directors <director>
            }
            ITERATOR film_second_csv <csvperrow> {
                FIELD country <country>
                FIELD directors <director>
                FIELD comment <comment>
            }
            EXPRESSION films <films_csv_file.film_csv >
            EXPRESSION films_name_csv <films_csv_file.film_csv.name>\s
            EXPRESSION films_name_year <films_csv_file.film_csv.name + "_" + films_csv_file.film_csv.year>

            :Films :[films.id] {
                :type :Film ;
                :name [films_name_csv] @en ;
                :year [films.year]  xsd:gYear ;
                :concate [films_name_year] xsd:gYear ;
                :country [films.country] ;
                :director [films.directors] ;
                :comment  [films.comment];
            }
            """;
    private static String RML_EXAMPLE = """
            @prefix rr: <http://www.w3.org/ns/r2rml#> .
            @prefix foaf: <http://xmlns.com/foaf/0.1/> .
            @prefix ex: <http://example.com/> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
            @prefix rml: <http://semweb.mmlab.be/ns/rml#> .
            @prefix ql: <http://semweb.mmlab.be/ns/ql#> .

            @base <http://example.com/base/> .

            <TriplesMap1>
              a rr:TriplesMap;

              rml:logicalSource [
                rml:source "student.csv";
                rml:referenceFormulation ql:CSV\s
              ];

              rr:subjectMap [
                rr:template "http://example.com/{Name}"
              ];
            
              rr:predicateObjectMap [
                rr:predicate foaf:name;
                rr:objectMap [
                  rml:reference "Name"
                ]
              ].
            """;

    @Test
    public void testTranslateShExML() throws TranslationException
    {
        String out = new Translator().translate(SHEXML_EXAMPLE, new Options("shexml")).plan();
        assertTrue(out.contains("<http://example.com/name>"));
        assertTrue(out.contains("<http://example.com/year>"));
        assertTrue(out.contains("<http://example.com/country>"));
        assertTrue(out.contains("<http://example.com/comment>"));
    }

    @Test
    public void testTranslateRML() throws TranslationException
    {
        Translation translation = new Translator().translate(RML_EXAMPLE, new Options("rml-io"));

        String out = translation.plan();
        assertTrue(out.contains("Source_0"));
        assertTrue(out.contains("ProjectionOp_1"));
        assertTrue(out.contains("ExtendOp_2"));
        assertTrue(out.contains("Serialize_3"));
        assertTrue(out.contains("Sink_4"));
        assertEquals(List.of(), translation.warnings());
    }

    @Test
    public void testBaseIriOption() throws TranslationException
    {
        String out = new Translator().translate(RML_EXAMPLE, new Options("rml-io").withBaseIri("http://other.org/")).plan();
        assertTrue(out.contains("http://other.org/"));
    }

    @Test
    public void testCanonicalTranslatorOption() throws TranslationException
    {
        String mapping = """
                @prefix rml: <http://w3id.org/rml/> .
                <http://example.com/tm> rml:subjectMap [ rml:constant <http://example.com/s> ] ;
                  rml:predicateObjectMap [ rml:predicate <http://example.com/p> ; rml:object "o" ] .
                """;
        String out = new Translator().translate(mapping, new Options("rml-kgc").withTranslator("canonical")).plan();
        // only a canonical plan carries a version, the algemaploom-rs version
        assertTrue(out, out.contains("\"version\":\""));
        assertTrue(out, out.contains("OneRecord"));
    }

    @Test
    public void testSyntaxErrorThrowsWithItsLocation()
    {
        TranslationException e = assertThrows(TranslationException.class,
                () -> new Translator().translate("<a> <b> .", new Options("rml-kgc")));
        ErrorRecord error = e.errors().get(0);
        assertEquals("syntax", error.kind());
        assertEquals("invalid-turtle", error.code());
        assertEquals(1, error.location().line());
    }

    @Test
    public void testUnknownLanguageIsRequestError()
    {
        TranslationException e = assertThrows(TranslationException.class,
                () -> new Translator().translate(RML_EXAMPLE, new Options("rml")));
        assertEquals("request", e.errors().get(0).kind());
        assertEquals("invalid-options", e.errors().get(0).code());
    }
}
