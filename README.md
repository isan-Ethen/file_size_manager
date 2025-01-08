# file_size_manager
Split and Merge files

# Usage
```sh
./file_size_manager [subcommand] [options]
```

## Subcommands
### split
This splits given files and put splitted files to a directory named sep-\<filename\>.

Options
- -o : Name output directory.
- -s : file size manager splits files according to this size.


### merge
This merge splitted files in the specified directories.

Options
- -o : Name output file.
