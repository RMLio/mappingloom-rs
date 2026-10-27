package be.ugent.idlab.knows.mappingLoom;

import java.io.IOException;
import java.util.List;
import java.util.stream.IntStream;

import org.json.JSONArray;
import org.json.JSONObject;

class Translator implements ITranslator {
	/**
	 * Takes the options object and returns the result document, both as JSON text.
	 */
	private static native String translateJson(String mapping, String options);

	static {
        try {
			String os = System.getProperty("os.name");
			String arch = System.getProperty("os.arch");
            String path;
            if (os.contains("Windows")) {
                path = "/Windows/translator.dll";
            } else if (os.contains("Linux")) {
                path = "/Linux/libtranslator.so";
            } else if (os.contains("Mac")) {
                if (arch.contains("aarch64") || arch.contains("arm")) {
                    path = "/Apple-aarch64/libtranslator.dylib";
                } else {
                    path = "/Apple-x86_64/libtranslator.dylib";
                }
            } else {
                path = "/" + os + "/translator.dll";
            }
            NativeUtils.loadLibraryFromJar(path);
        } catch (IOException e) {
            throw new RuntimeException("Cannot dynamically load Rust library: " + e);
        }
    }

	Translator() {

	}

	@Override
	public Translation translate(String mapping, Options options) throws TranslationException {
		JSONObject result = new JSONObject(translateJson(mapping, options.toJson()));
		List<ErrorRecord> warnings = records(result.getJSONArray("warnings"));
		if (result.has("errors")) {
			throw new TranslationException(records(result.getJSONArray("errors")), warnings);
		}
		return new Translation(result.getJSONObject("plan").toString(), warnings);
	}

	private static List<ErrorRecord> records(JSONArray array) {
		return IntStream.range(0, array.length()).mapToObj(array::getJSONObject).map(ErrorRecord::fromJson).toList();
	}
}
