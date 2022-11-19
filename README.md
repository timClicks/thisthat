# thisthat

Data format conversion utility.

## About

**thisthat** provides a command-line tool `tt` for converting between data formats.
It can to and from convert between these formats:

- JSON
- MsgPack
- YAML
- Pickle
- RON
- TOML

## Usage

`tt` reads from stdin and writes to stdout. Specify the formats using positional
parameters, e.g. `tt THIS THAT`. For example, to convert from JSON to TOML, use
`tt json toml`.

```console
$  echo '{"abc": 123 }' | tt json toml
abc = 123
```

To convert data from a file, use `cat` (or an equivalent tool) to do the reading.

```console
$ echo '{"abc": 123 }' > /tmp/example.json
$ cat /tmp/example.json | tt json toml
abc = 123
```

Some formats produce non-printable characters. Send the output to anther tool to
generate readable output.

```console
$  echo '{"abc": 123 }' | tt json msgpack | base64
gaNhYmN7
```
