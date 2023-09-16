
### Run locally

```sh
dfx start --clean # separate terminal

dfx deploy backend
dfx canister call backend version # should return v1

cargo run -p upload
dfx canister call backend version # should return v2
```

**Build v2:** `./src/backend/build-v2.sh`
