# generand

crate to generate random sequences from your string/vec/iterator symbols.

`--features=bin` to compile the cli tool `generand`:

```
USAGE:
    generand [OPTIONS] [DICTIONARY]

ARGS:
    <DICTIONARY>

OPTIONS:
    -c, --cats
    -d, --digits
    -f, --full
    -h, --hex
        --help               Print help information
    -n, --number <NUMBER>    [default: 1]
    -s, --size <SIZE>        [default: 12]
    -V, --version            Print version information
   ```

# EXAMPLE
```
$ generand -n 15                       
LE8Nd1gemH5g Pe17VgBKe74U 4bbNdQ16mWIp hxNMwlqoHD8W 3Wd2EEBKdhxw aVuXwSvUP4im 
Yhmx6KP2bdyI iincIf6FUdqp GrA5l5VlJJKT ldZBH2AlhDsf nTTYWvlXW5Ss e3FHFVNQ5PrO 
Wo97SaSIewBG O6OCdQVblgsH h1ypG5vMv65a 
```