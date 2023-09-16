
### Commands

```sh
dfx start --clean # separate terminal

dfx deploy backend
dfx canister call backend version # should return v1

dfx canister call aaaaa-aa update_settings '(record { canister_id = principal "'$(dfx canister id backend)'"; settings = record { controllers = opt vec { principal "'$(dfx identity get-principal)'"; principal "'$(dfx canister id backend)'"; }; }; })'
dfx canister status backend

cargo run -p upload
dfx canister call backend version # should return v2
```

**Build v2:** `./src/backend/build-v2.sh`

**Mainnet:** Run with `--network ic`
