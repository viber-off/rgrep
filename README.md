# Simple implemention of `grep` in Rust
> It is just a learning project for learning about Rust and fundamentals of the language.

# Usage

```sh
rgrep [options] pattern [path]
```

# Options

* `-h, --help` - print help message
* `-i, --ignore-case` - perform case-insensitive matching
* `-l, --line-numbers` - print line numbers along with the line 

# Examples

```sh
rgrep -i "hello" some_file.txt 
```
