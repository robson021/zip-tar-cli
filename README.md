## One CLI that supports multiple compression formats.
### Usage:
Run the program `cargo run` and follow the menu steps. 
<br>You can also run it with param: `cargo run -- [action] [path to file or directory]`
- extract all archives that will be found in current directory: `cargo run -- -xa .`
- extract specific: `cargo run -- -x "./my/dir/archive.zip"`
- tar: `cargo run -- -t "./my/dir/file.png"`
- zip all txt files in current dir: `cargo run -- -z "./*.txt"`
- zip with password encryption: `cargo run -- -ze "./my/dir/some_prefix*"`

### I prefer alias to use it anywhere:
`$ alias zip-tar-cli="cargoRun --manifest-path /Users/robert/git_repo/zip-tar-cli/Cargo.toml --"`
<br>
`$ zip-tar-cli -x ./some/dir/archive.zip`
