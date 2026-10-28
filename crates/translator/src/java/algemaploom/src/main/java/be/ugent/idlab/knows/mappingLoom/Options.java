package be.ugent.idlab.knows.mappingLoom;

import java.util.Map;

import org.json.JSONObject;

/**
 * The options of a translation request.
 *
 * @param language   The language of the mapping: {@code rml-kgc}, {@code rml-io}, {@code r2rml} or {@code shexml}.
 * @param baseIri    The base IRI of every triples map without {@code rml:baseIRI}, or null for the mapping's
 *                   {@code @base}.
 * @param optimizer  Switches optimizer rules on or off by their id; a rule without a toggle takes its default.
 * @param translator The translator of an {@code rml-kgc} mapping: {@code legacy} or {@code canonical}.
 */
public record Options(String language, String baseIri, Map<String, Boolean> optimizer, String translator) {
	public Options {
		optimizer = Map.copyOf(optimizer);
	}

	/**
	 * The options of a language, with the mapping's {@code @base}, the default optimizer rules and the legacy
	 * translator.
	 */
	public Options(String language) {
		this(language, null, Map.of(), "legacy");
	}

	public Options withBaseIri(String baseIri) {
		return new Options(language, baseIri, optimizer, translator);
	}

	public Options withTranslator(String translator) {
		return new Options(language, baseIri, optimizer, translator);
	}

	String toJson() {
		JSONObject json = new JSONObject().put("language", language).put("optimizer", optimizer)
				.put("translator", translator);
		if (baseIri != null) {
			json.put("base_iri", baseIri);
		}
		return json.toString();
	}
}
