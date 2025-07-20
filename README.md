One CLI that supports multiple compression formats.

### Usage
Run the program `cargo run` and follow the menu steps. 
<br>You can also run it with param: `cargo run -- [action] [path to file or directory]`
- extract all in path: `cargo run -- -xa .` [the most useful]
- extract: `cargo run -- -x "./my/dir/archive.zip"`
- zip: `cargo run -- -z "./my/dir/*.txt"`
- zip with password encryption: `cargo run -- -ze "./my/dir/*.txt"`
- tar: `cargo run -- -t "./my/dir/file.png"`
