## Usage Instructions

To quickly start deploying and testing this project, you can use the CasperLabs NCTL tool to deploy to a local, virtualized casper network. MAKE Services' NCTL Docker image works well and is up-to-date with the current upgrades of the Casper Network.

Start with:
```bash
docker pull makesoftware/casper-nctl
docker run --rm -it --name mynctl -d -p 11101:11101 -p 14101:14101 -p 18101:18101 makesoftware/casper-nctl
source nctl-activate.sh mynctl
```

#### Prepare for deployment
Execute these commands to prepare the contracts for deployment and obtain a secret key from the currently running NCTL container.

```bash
make prepare
make build-contract
nctl-view-user-secret-key user=1 > user1_secret_key.pem
```

#### Execute the smart contract deployment to NCTL
After deploying the contract, watch the deploy hash's status for the `execution_results` to be populated (meaning the blockchain has finalized and executed the contract)
```bash
casper-client put-deploy \
  --node-address  http://localhost:11101/rpc \
  --chain-name casper-net-1 \
  --secret-key user1_secret_key.pem \
  --payment-amount 30000000000 \
  --session-path contract/target/wasm32-unknown-unknown/release/inventory-count.wasm
  
  nctl-view-chain-deploy deploy=$DEPLOY_HASH
```

#### Confirm inventory / post deployment checks
You can now query the blockchain for any of the items from the inventory_items vec. Get a state-root-hash and the inventory_management_contract_hash from the NCTL network and then you can confirm some inventory count/key existence:
```bash
nctl-view-chain-state-root-hash node=1 | awk '{print $NF}'
STATE_ROOT_HASH= # copy/paste the output of above command
CONTRACT_HASH=$(nctl-view-user-account user=1 | grep "inventory_management_contract_hash" -B 1 | head -1 | awk '{print $NF}' | cut -d '"' -f2)
casper-client get-dictionary-item \
    --node-address http://localhost:11101 \
    --state-root-hash $STATE_ROOT_HASH \
    --contract-hash $CONTRACT_HASH \
    --dictionary-name inventory_management_dict \
    --dictionary-item-key "Nintendo Switch OLED"
```
Which should return a JSON object similar to the below output. The quantity is denoted in the CLValue 'parsed' field:
```bash
{
  "id": -5089817255395802256,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.0.0",
    "dictionary_key": "dictionary-223a76930a64a0aca0f067ed25f9a002b1a8dc72a6faeb133e3144bf316f9a62",
    "merkle_proof": "[1684 hex chars]",
    "stored_value": {
      "CLValue": {
        "bytes": "4b000000",
        "cl_type": "U32",
        "parsed": 75
      }
    }
  }
}
```

From here, you can feel free to experiment with increasing and decreasing the quantity of given items in inventory, or feel free to add new items to the inventory management system.
Increase Quantity of item (recieve more inventory):
```bash
nctl-view-chain-state-root-hash node=1 | awk '{print $NF}'
STATE_ROOT_HASH= # copy/paste the output of above command
CONTRACT_HASH=$(nctl-view-user-account user=1 | grep "inventory_management_contract_hash" -B 1 | head -1 | awk '{print $NF}' | cut -d '"' -f2)
casper-client get-dictionary-item \
    --node-address http://localhost:11101 \
    --state-root-hash $STATE_ROOT_HASH \
    --contract-hash $CONTRACT_HASH\
    --dictionary-name inventory_management_dict \
    --dictionary-item-key "Nintendo Switch OLED"
```
Decrease Quantity of item (sold some inventory):
```bash
casper-client put-deploy \
    --node-address http://localhost:11101 \
    --chain-name casper-net-1 \
    --secret-key user1_secret_key.pem \
    --payment-amount 5000000000 \
    --session-name "inventory_management_contract_hash" \
    --session-entry-point "inventory_dec_item" \
    --session-arg item:"string='Samsung 17\" Laptop'" \
    --session-arg dec_qty:"u32='125'"
```
Add Item:
```bash
casper-client put-deploy \
    --node-address http://localhost:11101 \
    --chain-name casper-net-1 \
    --secret-key user1_secret_key.pem \
    --payment-amount 5000000000 \
    --session-name "inventory_management_contract_hash" \
    --session-entry-point "inventory_add_item" \
    --session-arg item:"String='Apple iPad 9'" \
    --session-arg initial_qty:"U32='199'"
```





### Helpful NCTL/RPC query commands
For a full list of NCTL commands, visit the [NCTL commands GitHub page](https://github.com/casper-network/casper-node/blob/dev/utils/nctl/docs/commands.md).
nctl-activate.sh also offers tab completion of commands and displays all options available to the script.
```bash
nctl-view-node-secret-key node={1..5} # get secret key to wallet for validating node. 
nctl-view-user-secret-key user={1..10} # get secret key to wallet for on-chain user. 
nctl-view-chain-deploy deploy=X # monitor the status of your contract deployment
nctl-view-chain-state-root-hash # get updated state-root-hash
nctl-view-chain-block # view latest block
nctl-view-validator-account node={1..5} # view named-keys/account urefs
casper-client get-dictionary-item \
    --node-address http://localhost:11101 \
    --state-root-hash $STATE_ROOT_HASH \
    --contract-hash $CONTRACT_HASH \
    --dictionary-name inventory_management_dict \
    --dictionary-item-key "Nintendo Switch OLED"
```

nctl-activate.sh full list of available commands:
```bash
nctl-assets-dump                      nctl-erc20-fund-users                 nctl-stage-teardown                   nctl-view-chain-lfb
nctl-assets-ls                        nctl-erc20-install                    nctl-start                            nctl-view-chain-spec
nctl-assets-setup                     nctl-erc20-transfer                   nctl-start-after-n-blocks             nctl-view-chain-spec-accounts
nctl-assets-setup-from-stage          nctl-erc20-view-allowances            nctl-start-after-n-eras               nctl-view-chain-state-root-hash
nctl-assets-teardown                  nctl-erc20-view-balances              nctl-status                           nctl-view-faucet-account
nctl-assets-upgrade-from-stage        nctl-erc20-view-details               nctl-stop                             nctl-view-faucet-secret-key
nctl-auction-activate                 nctl-exec-upgrade-scenario-1          nctl-transfer                         nctl-view-node-config
nctl-auction-bid                      nctl-exec-upgrade-scenario-2          nctl-transfer-native                  nctl-view-node-error-log
nctl-auction-delegate                 nctl-exec-upgrade-scenario-3          nctl-transfer-native-batch-dispatch   nctl-view-node-finalisation-time
nctl-auction-undelegate               nctl-interactive                      nctl-transfer-native-batch-prepare    nctl-view-node-finalised-block-count
nctl-auction-withdraw                 nctl-join                             nctl-transfer-wasm                    nctl-view-node-log
nctl-await-n-blocks                   nctl-kv-storage-get-key               nctl-transfer-wasm-batch-dispatch     nctl-view-node-metrics
nctl-await-n-eras                     nctl-kv-storage-install               nctl-transfer-wasm-batch-prepare      nctl-view-node-peer-count
nctl-await-until-block-n              nctl-kv-storage-set-key               nctl-upgrade-protocol                 nctl-view-node-peers
nctl-await-until-era-n                nctl-leave                            nctl-view-chain-account               nctl-view-node-pending-deploy-count
nctl-clean                            nctl-ports                            nctl-view-chain-auction-info          nctl-view-node-ports
nctl-clean-logs                       nctl-processes                        nctl-view-chain-balance               nctl-view-node-rpc-endpoint
nctl-compile                          nctl-restart                          nctl-view-chain-balances              nctl-view-node-rpc-schema
nctl-compile-client                   nctl-rotate                           nctl-view-chain-block                 nctl-view-node-secret-key
nctl-compile-node                     nctl-stage-build                      nctl-view-chain-block-transfers       nctl-view-node-status
nctl-compile-node-launcher            nctl-stage-build-from-settings        nctl-view-chain-deploy                nctl-view-node-storage
nctl-contracts-hello-world-install    nctl-stage-init-settings              nctl-view-chain-era                   nctl-view-user-account
nctl-emergency-upgrade                nctl-stage-set-remote                 nctl-view-chain-era-info              nctl-view-user-secret-key
nctl-erc20-approve                    nctl-stage-set-remotes                nctl-view-chain-height                nctl-view-validator-account
```
