const test = require('./build')
// console.log(test.registerClipboardFormat("Rich Text Format1"))
// console.log(test.getClipboardFormatName(50032))
let formats = test.getClipboardFormats()
console.log(formats)
formats.forEach(
    value => {
        let data = test.getClipboardData(value.id)
        console.log(
            `${value.id},${value.name},,,,,,\n`,
            Array.from(data).slice(-8),
            `\n"\n`,
            data.toString('utf-8'),
            `\n"\n`,
            `\n\n`)
    }
)
// console.log(test.getClipboardFormats())