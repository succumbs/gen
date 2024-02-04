# gen
a simple &amp; secure cli password/passphrase generator

*this is mainly just a project to help me learn rust c:*


## usage
```
$ gen --help
a simple & secure cli password/passphrase generator

Usage: gen.exe <COMMAND>

Commands:
  passphrase  generate a passphrase
  password    generate a password
                  note: if no charset flags are provided, all charsets will be included
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
$ gen passphrase --help
generate a passphrase

Usage: gen.exe passphrase [OPTIONS]

Options:
  -l, --length <LENGTH>        length of the passphrase (in words) [default: 6]
  -s, --separator <SEPARATOR>  separator between words [default: -]
  -c, --capitalize             capitalize each word in the passphrase
  -h, --help                   Print help
```

```
$ gen password --help  
generate a password
note: if no charset flags are provided, all charsets will be included

Usage: gen.exe password [OPTIONS]

Options:
  -l, --length <LENGTH>  length of the password [default: 14]
  -a, --alpha            include alphabetical characters (a-z, A-Z)
  -n, --numeric          include numbers (0-9)
  -s, --special          include special characters (!@#$%^&*)
  -h, --help             Print help
```
