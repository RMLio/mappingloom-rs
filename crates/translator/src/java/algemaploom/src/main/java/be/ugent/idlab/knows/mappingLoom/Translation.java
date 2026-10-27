package be.ugent.idlab.knows.mappingLoom;

import java.util.List;

/**
 * A successful translation.
 *
 * @param plan     The plan as JSON text.
 * @param warnings What the translator left out or changed without failing.
 */
public record Translation(String plan, List<ErrorRecord> warnings) {
}
