build:
    cargo build

book:
    mdbook build book/book.toml --dest-dir ./docs

default:
    echo 'Hello, world!'
