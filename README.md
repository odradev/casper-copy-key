# Casper Copy Key

This is a simple Casper Session script that copies a value from source named key
to target named key.

To use it you have to define three arguments:
- `source_key` - the key from which the value will be copied,
- `target_key` - the key to which the value will be copied,
- `allow_overwrite_target` - a boolean value that determines if the target key
  can be overwritten.

Note that it copies only the `Key` not the value underneeth it. It means that
both `source_key` and `target_key` will point to the same value after the script
is executed.

## Preparation

Install wasm32 target.

```bash
rustup target add wasm32-unknown-unknown
```

## Build

Build the WASM file.

```bash
just build
```

## Use

Deploy the wasm file to the chain.

```bash
just deploy secret_key.pem "source_package_hash" "target_package_hash" false
```
