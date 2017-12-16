# Pig Latin RS

An English to Pig Latin converter writen in Rust.

## Usage

```
$ pig-latin words to translate
$ pig-latin
> words
> to translate
```

Any words passed as parameters will be translated. Use the parameter `-f` to specify a file as input. If not parameters are passed, `STDIN` will be read for input.

All output will be written to `STDOUT` while errors are written to `STDERR`.

## TODO

* [X] Use CLAP
* [X] Support read from stdin
* [X] Support read from file
* [X] Support read from arguments
* [ ] Add checks to ignore punctuation

## Licence

pig-latin-rs uses the [MIT](LICENCE) licence
