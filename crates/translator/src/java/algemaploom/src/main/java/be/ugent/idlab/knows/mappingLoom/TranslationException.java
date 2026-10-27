package be.ugent.idlab.knows.mappingLoom;

import java.util.List;
import java.util.stream.Collectors;

/**
 * A failed translation: at least one error, and the warnings found before.
 */
public class TranslationException extends Exception {
	private final List<ErrorRecord> errors;
	private final List<ErrorRecord> warnings;

	TranslationException(List<ErrorRecord> errors, List<ErrorRecord> warnings) {
		super(errors.stream().map(ErrorRecord::toString).collect(Collectors.joining("\n")));
		this.errors = List.copyOf(errors);
		this.warnings = List.copyOf(warnings);
	}

	public List<ErrorRecord> errors() {
		return errors;
	}

	public List<ErrorRecord> warnings() {
		return warnings;
	}
}
