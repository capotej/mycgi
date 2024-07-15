# mycgi

Personal static web server with CGI support meant strictly for internal use.

## Running

    RUST_LOG=info cargo run -- mycgi.example.toml

## Config

```
address = "localhost"
port = 8000
document_root = "private_html"

# localhost:8000/cgi/example
[bins.example]
# relative to document_root/cgi
path = "example.sh"
```