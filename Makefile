INSTALL_DIR=./ts/node_modules/zkwasm-ts-server/src/application
RUNNING_DIR=./ts/node_modules/zkwasm-ts-server

default: build

./src/admin.prikey: ./ts/node_modules/zkwasm-ts-server/src/init_admin.js
	node ./ts/node_modules/zkwasm-ts-server/src/init_admin.js ./src/admin.prikey

./ts/src/service.js:
	cd ./ts && npx tsc && cd -

build: ./src/admin.prikey ./ts/src/service.js
	wasm-pack build --release --out-name application --out-dir pkg
	wasm-opt -Oz -o $(INSTALL_DIR)/application_bg.wasm pkg/application_bg.wasm
	cp pkg/application_bg.wasm $(INSTALL_DIR)/application_bg.wasm
	#cp pkg/application.d.ts $(INSTALL_DIR)/application.d.ts
	#cp pkg/application_bg.js $(INSTALL_DIR)/application_bg.js
	cp pkg/application_bg.wasm.d.ts $(INSTALL_DIR)/application_bg.wasm.d.ts
	cd $(RUNNING_DIR) && npx tsc && cd -

clean:
	rm -rf pkg
	rm -rf ./src/admin.prikey

run:
	node ./ts/src/service.js

publish:
	node ./ts/node_modules/zkwasm-service-cli/dist/index.js addimage -n pump-elf -r "https://rpc.zkwasmhub.com:8090" -p "./pkg/application_bg.wasm" -u "0xd8f157Cc95Bc40B4F0B58eb48046FebedbF26Bde" -x "2763537251e2f27dc6a30179e7bf1747239180f45b92db059456b7da8194995a" -d "Multi User App" -c 22  > publish.log


deploy:
	docker build --file ./deploy/service.docker -t zkwasm-server . --network=host