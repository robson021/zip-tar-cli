One cli that supports multiple compression formats.

### Usage
Run the program `cargo run` and follow the menu steps or run it with param:<br/>
`cargo run -- [action] [path to file or directory]`<br/>
examples:
<br>extract: `cargo run -- -x "./my/dir/archive.zip"`
<br>extract all in path: `cargo run -- -xa "./some/path/dir"` [the most useful]
<br>zip: `cargo run -- -z "./my/dir/*.txt"`
<br>zip with password encryption: `cargo run -- -ze "./my/dir/*.txt"`
<br>tar: `cargo run -- -t "./my/dir/file.png"`
