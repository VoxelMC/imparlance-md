build-node:
	@wasm-pack build --target nodejs --out-dir nodepkg
	
build-npm:
	@wasm-pack build --target bundler --out-dir pkg 

test:
	@cd ./node && npm install ../nodepkg && node index.js && cd ..
