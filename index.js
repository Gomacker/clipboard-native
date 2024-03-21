const test = require('./build')
const {readFileSync, writeFileSync} = require("node:fs");
const {beep} = require("@napi-rs/cli/scripts");


let formats = test.getClipboardFormats()
console.log(formats)
formats.forEach(
    value => {
        try {
            let data = test.getClipboardData(value.id)
            console.log(
                `\n{ID: ${value.id}, Size: ${data.length}, Name: ${value.name ? value.name : "<None>"}}:\n`,
                Array.from(data),
            )
            switch (value.id) {
                case 1:  // CF_TEXT
                    console.log(test.ansi2utf8(data))
                    break
                case 13: // CF_UNICODETEXT
                    // 最好转换的方式，但是可能没有CF_TEXT可靠
                    console.log(data.toString('utf-16le'))
                    break
            }
            let path
            switch (value.name) {
                case 'HTML Format':
                    console.log(`HTML Format:\n${data.toString('utf-8')}`)
                    break
                case 'FileName':
                    path = data.toString('utf-8').split('\0')[0]
                    console.log('FileName:', path)
                    break
                case 'FileNameW':
                    path = data.toString('utf-16le').split('\0')[0]
                    console.log('FileNameW:', path)
                    break
                case 'UniformResourceLocator':
                    console.log('UniformResourceLocator:', data.toString('utf-8'))
                    break
            }
        }catch (e) {
            console.log(`Exception on ${e}`)
        }
    }
)
// console.log(test.getClipboardFormats())