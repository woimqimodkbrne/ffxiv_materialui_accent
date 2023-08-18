if [ "$1" = "client" ] && [ "$2" = "release" ]; then
	echo "client release"
	cargo run --release --manifest-path=./client/Cargo.toml
elif [ "$1" = "client" ]; then
	echo "client dev"
	cargo run --manifest-path=./client/Cargo.toml
elif [ "$1" = "release" ]; then
	echo "plugin release"
	cargo build --lib --release --manifest-path=./plugin/Cargo.toml
	dotnet build ./plugin/plugin/Aetherment.csproj
	mv ./plugin/plugin/bin/Release/Aetherment.dll ./target/release/Aetherment.dll
else
	echo "plugin dev"
	cargo build --lib --manifest-path=./plugin/Cargo.toml
	dotnet build ./plugin/plugin/Aetherment.csproj
	mv ./plugin/plugin/bin/Release/Aetherment.dll ./target/debug/Aetherment.dll
fi