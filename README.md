# External Lab: Create a File Size Formatter

## Assignment
In this lab, you will enhance a size formatter application in Rust. 
Use the [example code](https://github.com/alfredodeza/rust-structs-types-enums/blob/main/examples/14-match-enums/match-enum/src/main.rs) to get started and get an idea on how to use enum and match to handle different sizes. You are tasked with extending the application to allow a user to pass in a String representing size and unit, and then returning a debug representation of a struct that shows all the different representations in KB, MB, and GB . 
This is an example that takes an input and provides the output required:
```shell
$ cargo run "24 mb"
Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }
```

## Output

`fsize-formatter` will print the equivelant binary size for bytes, kilobytes, megabytes and gigabytes. The input must be a numerical value followed by a unit and enclosed in quotes, i.e. "100 mb". The unit value is case-insensitive, so (for instance) "100 MB" is also valid. If you enter any unit value other than "b", "kb", "mb" or "gb" (in upper- or lowercase), then you get an error. If you forget to enclose the input in quotes (`""`) you also get an error with a reminder to format the input correctly.
