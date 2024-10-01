export FLATPAK_ID := "com.github.LegitCamper.Rust-Browser"

ultralight-version := "1.3.0-linux-x64"
ultralight-file := "ultralight-sdk-" + ultralight-version + ".7z"
ultralight-url := "https://ultralight-files.sfo3.cdn.digitaloceanspaces.com/" + ultralight-file

build:
    cargo build

run:
    cargo run

release:
    cargo build --release

release-lto:
    cargo build --profile release-lto

@create-flatpak-sources:
    ./flatpak/venv/bin/python3 ./flatpak/flatpak-cargo-generator.py ./Cargo.lock -o ./flatpak/cargo-sources.json

@get-ultralight: clean-ultralight
    mkdir ultralight
    wget {{ultralight-url}}
    7z x {{ultralight-file}} bin
    7z x {{ultralight-file}} resources
    rm {{ultralight-file}}
    mv bin resources ultralight

@clean-ultralight:
    rm -rf bin
    rm -rf resources
    rm -rf ultralight

@build-flatpak: create-flatpak-sources
    flatpak-builder --user build-dir ./flatpak/$FLATPAK_ID.json

@install-flatpak:
    flatpak-builder --user --install build-dir ./flatpak/$FLATPAK_ID.json

run-flatpak: 
    flatpak run --user $FLATPAK_ID

export-flatpak: build-flatpak
    flatpak build-export export build-dir/
    flatpak build-bundle export Rust-Browser.flatpak $FLATPAK_ID
