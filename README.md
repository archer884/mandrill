# mandrill

> a cli for testing mandrill templates

It should be noted that this tool was written for a very specific use case. You probably don't have the problem it solves--and, if you do, you should solve it some other way if possible. However, this tool does have one generally applicable command: you can use it to generate previews with replacements. Neat, huh?

## Usage

This tool expects your Mandrill API key to be provided in a file that is referenced in an environment variable named `MANDRILL_SECRET_PATH`. If I understood the docs I read online correctly, doing this (along with some other linuxy things that I'm not responsible for) is a decent way to keep your API key out of your bash history, etc.

...The tool will also read an API key directly from `MANDRILL_SECRET_PATH`, but why would you want to do that? Don't do that. I worked hard on the thing that reads keys from a text file...

> Note: the actual intent here is for you to store your key in some encrypted fashion and provide the key as a file descriptor rather than as a plain text file. *That* was my understanding of the document I read. Exactly how to do that, however, was left as an exercise to the reader, and so you'll just have to let me know if this works for that or not.

### Inspect

`mandrill inspect <template name>`

This will spew the given template all over your terminal. It's probably advisable to redirect standard out to a file somewhere.

### Render

`mandrill render <template name> [--var <replacement key>:<replacement value> ...]`

This will again spew the given template all over your terminal. Unlike the other option, this one will replace your template placeholders with whatever you ask it to.

### Fix

`mandrill fix <template name>`

This does not spew templates all over your screen. What it does is to download the template, check for spurious `MC_` placeholders, remove them, and publish the modified template. This is (clearly, right?) a destructive operation--it will mess up your template if any of your `MC_` placeholders were not spurious. Additionally, you only need to do this if, for some reason, you can't simply import templates in Handlebars mode.

Don't paint yourself into this corner, my son.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE][apc] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[apc]:https://github.com/archer884/mandrill/blob/master/LICENSE-APACHE
[mit]:https://github.com/archer884/mandrill/blob/master/LICENSE-MIT
