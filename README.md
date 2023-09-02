# tdrip

tdrip is a command-line tool to easily remove headers and metadata from text.

## Usage

```sh
$ cat example.txt
This is the example text.
TODO This line will be removed.
FIXME This line will be removed.
HACK: This line will be removed.
NOTEThis line will be removed.
This line will not be removed. TODO

$ tdrip example.txt
This is the example text.
This line will not be removed. TODO
```

## What lines are removed?

The lines starts with the following annotations.

- FIXME
- HACK
- NOTE
- TODO
- XXX
