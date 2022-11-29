import {CosmWasmClient} from "@cosmjs/cosmwasm-stargate";

async function getContractAddresses() {
    let wasmClient = await CosmWasmClient.connect(TENDERMINT_RPC);
    let codes = await wasmClient.getCodes();
    let contracts = await wasmClient.getContracts(codes[0].id);

    wasmClient.disconnect()
    return contracts
}

// async function getTickets(address: string) {
//     let wasmClient = await CosmWasmClient.connect(TENDERMINT_RPC);
//     wasmClient.
// }