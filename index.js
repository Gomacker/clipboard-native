const test = require('./build')
const {readFileSync, writeFileSync} = require("node:fs");
const os = require("os");
const {onClipboardUpdate} = require("./build");


let formats = test.getClipboardFormats()
console.log(formats)
formats.forEach(
    value => {
        try {
            let data = test.getClipboardData(value.id)
            console.log(
                `\n{ID: ${value.id}, Size: ${data.length}, Name: ${value.name ? value.name : "<None>"}}:\n`,
                Array.from(data),
                Array.from(data).splice(-8)
            )
            switch (value.id) {
                case 1:  // CF_TEXT
                    console.log(test.ansi2utf8(data))
                    break
                case 13: // CF_UNICODETEXT
                    console.log(data.toString('utf-16le'))
                    break
                case 15: // CF_HDROP
                    // console.log(data.toString('utf-16le'))
                    break
                case 17: // CF_DIBV5
                    // console.log(data.toString('utf-16le'))
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
                case 'QQ_Unicode_RichEdit_Format':
                    console.log('QQ_Unicode_RichEdit_Format:', data.toString('utf-8'))
                    break
            }
        }catch (e) {
            console.log(`Exception on ${e}`)
        }
    }
)

onClipboardUpdate(() => {
    console.log('Clipboard updated jssssssssssssssssssssssssss')
})

process.stdin.on('data', data => {
    if (data.toString() === '\r\n' || data.toString() === '\n') {
        console.log('exit...')
        process.exit(0)
    }
    console.log('data:', data.toString().trim())
})

// console.log(test.getClipboardFormats())