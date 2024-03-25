# Clipboard-native

a node addon that allows you to interact with the clipboard using the windows api

written in rust using napi-rs

## Simple Example

```javascript
const {onClipboardUpdate, getClipboardFormats, getClipboardData} = require('build');

// onClipboardUpdate will call the callback function when the clipboard is updated
onClipboardUpdate(() => {
    // get all the formats that are currently on the clipboard
    const formats = getClipboardFormats()
    formats.forEach((format) => {
        console.log(format)
        // this will return Buffer of the data
        const data = getClipboardData(format.id)
        console.log(data)
    })
});
```
