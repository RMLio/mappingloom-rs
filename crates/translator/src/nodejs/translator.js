const fs = require('fs')
const translator = require('.')

if (process.argv.length !== 3) {
    console.error('Usage: node translator.js <language> < mapping')
    process.exit(2)
}
try {
    const translation = translator.translate(fs.readFileSync(0).toString(), {language: process.argv[2]})
    translation.warnings.forEach(warning => console.error(warning))
    console.log(translation.plan)
} catch (e) {
    if (e.name !== 'TranslationError') throw e
    console.error(e.message)
    process.exit(1)
}
