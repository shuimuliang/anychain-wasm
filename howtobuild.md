```sh
wasm-pack build --target web
```

cp -r pkg <dest-project>/pkg, e.g:
```sh
cp -r pkg ~/anychain/anychain-wasm-vite/pkg
```

after pnpm i, cp pkg/anychain_wasm_bg.wasm <dest-project>/node_modules/.vite/deps/  
```sh
cp pkg/anychain_wasm_bg.wasm ~/anychain/anychain-wasm-vite/node_modules/.vite/deps/
```
