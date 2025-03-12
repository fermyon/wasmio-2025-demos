.PHONY: build-components build-manual-composition-demo compose-apps

build-components:
	cd components/merge-sorter && cargo component build
	cd components/bubble-sorter && jco componentize index.js --wit wit/world.wit --disable all -o bubble-sorter.wasm 

build-manual-composition-demo:
	cd manual-composition-demo && cargo component build

compose-apps:
	wac plug --plug ./components/merge-sorter/target/wasm32-wasip1/debug/merge_sorter.wasm ./manual-composition-demo/target/wasm32-wasip1/debug/manual-composition-demo.wasm -o dist/merge-sorter.wasm
	wac plug --plug ./components/bubble-sorter/bubble-sorter.wasm ./manual-composition-demo/target/wasm32-wasip1/debug/manual-composition-demo.wasm -o dist/bubble-sorter.wasm