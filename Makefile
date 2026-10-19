.PHONY: dev release clean check

dev:
	wasm-pack build --dev --target web --out-dir pkg
	cp web/index.html pkg/index.html
	cd pkg && python3 -m http.server 8081

release:
	wasm-pack build --release --target web --out-dir pkg

clean:
	cargo clean

check:
	cargo clippy --target wasm32-unknown-unknown --all-features -- -D warnings
