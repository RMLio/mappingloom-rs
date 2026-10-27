package be.ugent.idlab.knows.mappingLoom;

import java.util.List;

import org.json.JSONArray;
import org.json.JSONObject;

/**
 * An error or a warning of a translation.
 *
 * @param kind     {@code request}, {@code syntax}, {@code mapping}, {@code unsupported} or {@code internal}.
 * @param code     A stable identifier within its kind.
 * @param message  For humans.
 * @param origin   The mapping node it concerns, as an IRI or skolem IRI, or null.
 * @param location The line and column in the mapping text, or null.
 * @param causes   The chain of underlying messages, outermost first.
 */
public record ErrorRecord(String kind, String code, String message, String origin, Location location,
		List<String> causes) {

	/**
	 * A line and column in the mapping text, both counted from 1.
	 */
	public record Location(long line, long column) {
	}

	static ErrorRecord fromJson(JSONObject json) {
		JSONObject location = json.optJSONObject("location");
		return new ErrorRecord(
				json.getString("kind"),
				json.getString("code"),
				json.getString("message"),
				json.optString("origin", null),
				location == null ? null : new Location(location.getLong("line"), location.getLong("column")),
				json.optJSONArray("causes", new JSONArray()).toList().stream().map(String::valueOf).toList());
	}

	@Override
	public String toString() {
		StringBuilder text = new StringBuilder(kind + "/" + code + ": " + message);
		if (location != null) {
			text.append(" (line ").append(location.line()).append(", column ").append(location.column()).append(")");
		}
		if (origin != null) {
			text.append(" <").append(origin).append(">");
		}
		causes.forEach(cause -> text.append("\n  caused by: ").append(cause));
		return text.toString();
	}
}
