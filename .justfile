user     := "atareao"
name     := `basename ${PWD}`
version  := `vampus show`

list:
    @just --list

install:
    cargo install --path .

upgrade:
    @vampus upgrade --patch

release:
    @git add .
    @git commit -m ":bookmark: Release v{{version}}"
    @git tag -a v{{version}} -m "Release v{{version}}"
    @git push origin main
    @git push origin v{{version}}

    @gh release create v{{version}} \
        --title "Release v{{version}}" \
        --notes "Release v{{version}}"

