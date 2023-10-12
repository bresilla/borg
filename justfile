build:
    cargo build

book:
    mdbook build book --dest-dir ../docs

default:
    echo 'Hello, world!'
