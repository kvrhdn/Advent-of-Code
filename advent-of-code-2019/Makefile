test:	test_cargo test_wasm build

test_cargo:
	cargo test

test_wasm:
	for dir in day*; do															\
		echo "\n* wasm-pack test $$dir\n";										\
		wasm-pack test --firefox --headless $$dir;								\
	done

build:
	for dir in day*; do															\
		echo "\n* wasm-pack build $$dir\n";										\
		wasm-pack build $$dir;													\
	done
