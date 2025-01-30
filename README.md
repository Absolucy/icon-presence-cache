## wtf is this

this is just a doohickey I made to automatically pre-calculate an initial cache for `/proc/icon_exists` in SS13.

it's simple enough to use, just do something like this `generate-icon-exists-cache -i "C:\Users\Lucy\Code\SS13\MonkeStation" -o output.json`

include `-p` or `--pretty` if you want pretty JSON rather than minified JSON

the output JSON will be in this general format:
```json
{
	"icons/a.dmi": ["foo", "bar"],
	"icons/b.dmi": ["mrrp", "mrrrow"]
}
```

if you want typecache-style "associative" lists like this, you can include `-a` or `--assoc` in the command:
```json
{
	"icons/a.dmi": {
		"foo": true,
		"bar": true
	},
	"icons/b.dmi": {
		"mrrp": true,
		"mrrrow": true
	}
}
```

## License


Copyright (c) `2025` `Lucy <lucy@absolucy.moe>`

This software is provided 'as-is', without any express or implied warranty. In
no event will the authors be held liable for any damages arising from the use of
this software.

Permission is granted to anyone to use this software for any purpose, including
commercial applications, and to alter it and redistribute it freely, subject to
the following restrictions:

1.  The origin of this software must not be misrepresented; you must not claim
    that you wrote the original software. If you use this software in a product,
    an acknowledgment in the product documentation would be appreciated but is
    not required.

2.  Altered source versions must be plainly marked as such, and must not be
    misrepresented as being the original software.

3.  This notice may not be removed or altered from any source distribution.
