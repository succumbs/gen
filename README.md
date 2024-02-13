# gen
a simple &amp; secure cli password/passphrase generator

*this is mainly just a project to help me learn rust c:*


## usage
```
$ gen --help
a simple & secure cli password/passphrase generator

Usage: gen <COMMAND>

Commands:
  passphrase  generate a passphrase [aliases: pp]
  password    generate a password
                  will include all charsets unless explicitly specified by flags [aliases: pw, pwd]
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
$ gen passphrase --help
generate a passphrase

Usage: gen passphrase [OPTIONS]

Options:
  -l, --length <LENGTH>        length of the passphrase (in words) [default: 6]
  -s, --separator <SEPARATOR>  separator between words [default: -]
  -c, --capitalize             capitalize each word in the passphrase
  -h, --help                   Print help
```

```
$ gen password --help
generate a password
will include all charsets unless explicitly specified by flags

Usage: gen password [OPTIONS]

Options:
  -l, --length <LENGTH>    length of the password [default: 14]
  -l, --letters            include letters
  -d, --digits             include digits
  -s, --specials           include special characters
  -e, --exclude-ambiguous  exclude ambiguous characters
  -h, --help
```