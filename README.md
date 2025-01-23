# ZKWASM application demo

## Prepare environment

For fresh environment, run `source script/environment_linux.sh` for Linux.

If you see some error messages, need manually install the error module in your OS.

## Start Redis

`redis-server`

## Start Mongodb

```
mkdir db
mongod --dbpath db
```

## Start dbservice
Clone the zkwasm mini rollup:
```
git clone git@github.com:DelphinusLab/zkwasm-mini-rollup.git
```
In `./dbservice`, run `bash run.sh`

# Install & Run Wasm Service

```
git clone https://github.com/riddles-are-us/zkwasm-automata
```

## Install zkwasm-ts-server
In `./ts`, run:
```
npm install
npx tsc
```

## Build WASM image:
In './', run:
```
make
```

## Run WASM service:
```
node ts/node_modules/zkwasm-ts-server/src/service.js
```
