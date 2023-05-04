# MetaExchange CosmWasm contract

This contract contains the logic used to receive the loans once they are bridged, as well as the logic in charge of sending unhealthy loans back to Ethereum. As executing code via the Peggy bridge is not currently possible, this contract aims to provide a business case for Injective to add data execution.

The mtxch cosmwasm contract currently has two main functionalities: 

* Receive new bridged loans (positions) and store them. 
* Run a sudo function (begin_blocker) that evaluates the health of the loans and automatically returns unhealthy positions back to Ethereum. 