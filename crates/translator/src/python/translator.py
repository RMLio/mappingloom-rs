import sys
import ltranslator

if __name__ == '__main__':
    if len(sys.argv) != 2:
        sys.exit('Usage: python translator.py <language> < mapping')
    try:
        translation = ltranslator.translate(sys.stdin.read(), {'language': sys.argv[1]})
    except ltranslator.TranslationError as e:
        sys.exit(str(e))
    for warning in translation.warnings:
        print(warning, file=sys.stderr)
    print(translation.plan)
