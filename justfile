build:
    cargo build

book:
    mdbook build book --dest-dir ../docs

release type:
    cargo release version {{type}}
    git cliff -u -t $(cat Cargo.toml | grep version | head -1 | choose 2 | tr -d ,\") --prepend CHANGELOG.md
    gh release create $(cat Cargo.toml | grep version | head -1 | choose 2 | tr -d ,\")"

default:
    echo 'Hello, world!'
