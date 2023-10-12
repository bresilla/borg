build:
    cargo build

book:
    mdbook build book --dest-dir ../docs

deploy type:
    #!/usr/bin/env bash
    cargo release version {{type}} --execute
    version=$(cat Cargo.toml | grep version | head -1 | choose 2 | tr -d ,\")
    git tag -a $version -m $version
    git cliff -u -t $version --prepend CHANGELOG.md
    git add --all && git commit -m $version
    git push --follow-tags
    gh release create $version

default:
    echo 'Hello, world!'
