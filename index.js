const test = require('./build')
const {readFileSync, writeFileSync} = require("node:fs");
const os = require("os");
const {onClipboardUpdate, ansi2utf8} = require("./build");


function logClipboard() {
    let formats = test.getClipboardFormats()
    console.log(formats, '\n')
    formats
        .filter(value => [
            'FileName',
            'FileNameW',
            'FileGroupDescriptor',
            'FileGroupDescriptorW',
        ].includes(value.name))
        .forEach(
            value => {
                try {
                    let data = test.getClipboardData(value.id)
                    console.log(`{ID: ${value.id}, Size: ${data.length}, Name: ${value.name ? value.name : "<None>"}}:`)
                    console.log(Array.from(data))
                    // console.log(Array.from(data).splice(-8))

                    let content = ''
                    switch (value.id) {
                        case 1:  // CF_TEXT
                            content = `\x1b[2;3;32m${ansi2utf8(data)}\x1b[0m`
                            // console.log(test.ansi2utf8(data))
                            break
                        case 7: // CF_OEMTEXT
                            content = `\x1b[2;3;32m${ansi2utf8(data)}\x1b[0m`
                            // console.log(data.toString('utf-8'))
                            break
                        case 13: // CF_UNICODETEXT
                            content = `\x1b[2;3;32m${data.toString('utf-16le')}\x1b[0m`
                            // console.log(data.toString('utf-16le'))
                            break
                        case 15: // CF_HDROP
                            // content = `\x1b[2;3;32m${data.toString('utf-8').split('\0').join('\n')}\x1b[0m`
                            content = `\x1b[2;3;32m${data.toString('utf-16le')}\x1b[0m`
                            // console.log(data.toString('utf-16le'))
                            break
                        case 17: // CF_DIBV5
                            // console.log(data.toString('utf-16le'))
                            break
                    }
                    // let path
                    switch (value.name) {
                        case 'HTML Format':
                            content = `\x1b[2;3;32m${data.toString('utf-8')}\x1b[0m`
                            break
                        case 'FileName':
                            content = `\x1b[2;3;32m${data.toString('utf-8').split('\0').join('\n')}\x1b[0m`
                            break
                        case 'FileNameW':
                            content = `\x1b[2;3;32m${data.toString('utf-16le').split('\0').join('\n')}\x1b[0m`
                            break
                        case 'UniformResourceLocator':
                            content = `\x1b[2;3;32m${data.toString('utf-8')}\x1b[0m`
                            break
                        case 'QQ_Unicode_RichEdit_Format':
                            content = `\x1b[2;3;32m${data.toString('utf-8')}\x1b[0m`
                            break
                        case 'FileGroupDescriptorW':
                            content = `\x1b[2;3;32m${data.toString('utf-16le').split('\0').join('\n')}\x1b[0m`
                            break
                        case 'FileGroupDescriptor':
                            content = `\x1b[2;3;32m${data.toString('utf-8').split('\0').join('\n')}\x1b[0m`
                            break
                        case 'x-special/gnome-copied-files':
                            content = `\x1b[2;3;32m${data.toString('utf-8')}\x1b[0m`
                            break
                        case 'QQCapture_CLIPBOARDFORMAT':
                            content = `\x1b[2;3;32m${data.toString('utf-8')}\x1b[0m`
                            break
                        case 'Rich Text Format':
                            content = `\x1b[2;3;32m${data.toString('utf-8')}\x1b[0m`
                            break
                        case 'JAVA_DATAFLAVOR:application/x-java-jvm-local-objectref; class=com.intellij.codeInsight.editorActions.FoldingData':
                            content = `\x1b[2;3;32m${data.toString('utf-8')}\x1b[0m`
                            break
                        case 'JAVA_DATAFLAVOR:application/x-java-serialized-object; class=com.intellij.openapi.editor.impl.EditorCopyPasteHelperImpl$CopyPasteOptionsTransferableData':
                            content = `\x1b[2;3;32m${data.toString('utf-8')}\x1b[0m`
                            break
                    }
                    if (content) console.log(content)
                    else console.log(`\x1b[1;31m[undefined format decoder or binary format]\x1b[0m`)
                    console.log('\n')
                } catch (e) {
                    console.log(`Exception on ${e}`)
                }
            }
        )
}

logClipboard()
onClipboardUpdate(() => {
    logClipboard()
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