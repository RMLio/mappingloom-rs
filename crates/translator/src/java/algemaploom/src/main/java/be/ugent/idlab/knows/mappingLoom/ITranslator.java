package be.ugent.idlab.knows.mappingLoom;

public interface ITranslator {
	/**
	 * Translate a mapping document into a MappingLoom plan.
	 *
	 * @param mapping The mapping document, in the language that the options name.
	 * @param options The options of the request.
	 * @return The plan and the warnings.
	 * @throws TranslationException When the options are invalid or the mapping can't be translated.
	 */
	Translation translate(String mapping, Options options) throws TranslationException;

	/**
	 * A default instance of the translator.
	 *
	 * @return A new ITranslator instance
	 */
	static ITranslator getInstance() {
		return new Translator();
	}
}
