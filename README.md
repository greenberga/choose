# `choose`

`choose` is a command line utility that randomly selects items from a list passed on standard input. It takes as arguments the number of items to choose as well as the character(s) used to delimit list items.

## Examples

```
$ seq 100 | choose 5
67
69
7
38
38
```

```
$ echo -e 'A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z' | choose 3 -d ,
T
D
E
```

## Help

```
./target/release/choose --help
choose 0.1.0
Aaron Greenberg <p@aaronjgreenberg.com>
a command line tool for random selection from a list

USAGE:
    choose [OPTIONS] [count]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <delimiter>        list delimiter [default: 
                          ]

ARGS:
    <count>    number of items to choose [default: 1]
```
