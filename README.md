# rust_password
rust password generator

# Usage
```bash
./password_generator -h
password_generator 0.0.1
Jérémie Bellavance <jbellavance34@gmail.com>

USAGE:
    password_generator [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -l, --lenght <LENGHT>    lenght of the password [default: 10]
    -n, --numbers            activates numbers
    -s, --special-char       activates special characthers
    -V, --version            Print version information
  
```
  
# build release
```bash
cargo build --release
```
  
# Exemple run
```bash
./password_generator -l 20 -n 
Generated password: "h0uiwn9gthyer8y5172z"
```
