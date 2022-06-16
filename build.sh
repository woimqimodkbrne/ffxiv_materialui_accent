RUST_BACKTRACE=1
cargo build --lib --release --manifest-path=./aetherment/Cargo.toml
# -f on mv does fuckall
rm $APPDATA/XIVLauncher/devPlugins/Aetherment/aetherment_core.dll
mv ./target/release/aetherment_core.dll $APPDATA/XIVLauncher/devPlugins/Aetherment/aetherment_core.dll

dotnet build ./aetherment/plugin/Aetherment.csproj