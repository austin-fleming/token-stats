## References
- https://www.gakonst.com/ethers-rs/providers/http.html

## CLI
- Get current mempool from quicknode:
    ```curl --data '{"method":"parity_pendingTransactions","params":[],"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST <node-url>```
    - Spec: https://openethereum.github.io/JSONRPC-parity-module#parity_pendingtransactions
    - Examples: https://www.quicknode.com/guides/ethereum-development/transactions/how-to-access-ethereum-mempool
    ```$ curl --data '{"method":"txpool_content","id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST <node-url>```
- Get more summarized version:
    ```$ curl --data '{"method":"txpool_inspect","id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST <node-url>```