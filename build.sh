if [ "$1" = "release" ]; then
	echo "release"
	cargo build --lib --release --manifest-path=./aetherment/Cargo.toml
	# -f on mv does fuckall
	rm $APPDATA/XIVLauncher/devPlugins/Aetherment/aetherment_core.dll
	mv ./target/release/aetherment_core.dll ./build/aetherment_core.dll
	dotnet build ./aetherment/plugin/Aetherment.csproj
else
	echo "dev"
	cargo build --lib --profile dev_r --manifest-path=./aetherment/Cargo.toml
	# RUSTFLAGS='-C prefer-dynamic' cargo build --lib --profile dev_r --manifest-path=./aetherment/Cargo.toml
	rm $APPDATA/XIVLauncher/devPlugins/Aetherment/aetherment_core.dll
	mv ./target/dev_r/aetherment_core.dll ./build/aetherment_core.dll
	dotnet build ./aetherment/plugin/Aetherment.csproj
fi