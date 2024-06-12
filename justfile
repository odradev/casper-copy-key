default:
    just -l

build:
    cargo build --target wasm32-unknown-unknown --release

deploy SECRET_KEY SOURCE TARGET ALLOW_OVERWRITE_TARGET:
    casper-client put-deploy \
        --node-address http://95.216.37.50:7777 \
        --chain-name casper-test \
        --secret-key {{SECRET_KEY}} \
        --payment-amount 300000000 \
        --session-path target/wasm32-unknown-unknown/release/casper-copy-key.wasm \
        --session-arg "source_key:string='{{SOURCE}}'" \
        --session-arg "target_key:string='{{TARGET}}'" \
        --session-arg "allow_overwrite_target:bool='{{ALLOW_OVERWRITE_TARGET}}'"
