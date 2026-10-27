package be.ugent.idlab.knows.mappingLoom;

import java.util.Map;

import org.json.JSONObject;

/**
 * The options of a translation request.
 *
 * @param language  The language of the mapping: {@code rml-kgc}, {@code rml-io}, {@code r2rml} or {@code shexml}.
 * @param baseIri   The base IRI of every triples map without {@code rml:baseIRI}, or null for the mapping's
 *                  {@code @base}.
 * @param optimizer Switches optimizer rules on or off by their id; a rule without a toggle takes its default.
 */
public record Options(String language, String baseIri, Map<String, Boolean> optimizer) {
	public Options {
		optimizer = Map.copyOf(optimizer);
	}

	/**
	 * The options of a language, with the mapping's {@code @base} and the default optimizer rules.
	 */
	public Options(String language) {
		this(language, null, Map.of());
	}

	public Options withBaseIri(String baseIri) {
		return new Options(language, baseIri, optimizer);
	}

	String toJson() {
		JSONObject json = new JSONObject().put("language", language).put("optimizer", optimizer);
		if (baseIri != null) {
			json.put("base_iri", baseIri);
		}
		return json.toString();
	}
}
