_Minimalistic easy-to-understand encryption tool._

[![License: GPL3](https://img.shields.io/badge/License-GNU%20GPL-blue)](https://opensource.org/license/gpl-3-0)
[![Crate](https://img.shields.io/crates/v/hyperlink-one-time-pad.svg)](https://crates.io/crates/hyperlink-one-time-pad)

# hyperlink-one-time-pad

## Example

```bash
hyperlink-one-time-pad "secret_stuff.zip" add http://example.com/vid.mp4 http://example.com/data.rar
```

## Description

The above example uses two files from the internet (both should have a bigger file size than the file to encrypt) that are both downloaded automatically and "layed over" the file to encrypt. Decrypting works the same way using the parameter sub instead of add. You only need to memorize the files that are online available and don't need to store or exchange huge keys, which is a negative point with the normal one-time-pad. It is also possible to use local files as keys instead of hyperlinks.

## Notes

- Remember that the internet providers may safe the files you download. Use this on top of normal encryption methods.
- The order of the key parameters does not matter.
- Hyperlinks must start with http:// or https://.

# Build

You might need to install ```openssl-devel```.
