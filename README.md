# minigrep
* A lightweight implementation of the Global Regular Expression Print command-line utility. In other words, this is a tool that can be used to search for text/regex patterns inside of files.

## Development Environment:
* Lenovo T14 Gen 6 (x86_64)
* Ubuntu (24.04.3)
* GNOME Terminal (3.52.0)
* Vi IMproved (9.1)

## Dependencies:
* Git (2.43.0 or newer)
* Cargo (1.92.0 or newer)

## Target Environment
* A modern PC (x86_64) running an Ubuntu (24.04.3) instance 

# How to Run

1. Clone the repo
```
git clone https://github.com/lucasthormann/minigrep.git
```

2. Run the following command to use the tool
```
cargo run --release -- *query* *path to file* > output.txt
```
i.e.
```
cargo run --release -- know src/poem.txt > output.txt
```

3. View contents of the output file
```
cat output.txt
```
