import { useEffect, useState } from 'react';
import * as SecureStore from "expo-secure-store";
import {SigningStargateClient} from '@cosmjs/stargate';
import {DirectSecp256k1HdWallet} from "@cosmjs/proto-signing";

async function save(key: string, value: string) {
    await SecureStore.setItemAsync(key, value);
}

async function getValueFor(key: string) {
    const value = await SecureStore.getItemAsync(key);
    return value || "";
}

async function getWalletClient() {
    const wallet = await getOrCreateWallet();
    const rpcEndpoint = "https://rpc.my_tendermint_rpc_node";
    return await SigningStargateClient.connectWithSigner(rpcEndpoint, wallet);
}

async function getOrCreateWallet() {
    let mnemonic = await getValueFor(COSMOS_MNEMONIC_KEY);

    let wallet: DirectSecp256k1HdWallet;
    if (mnemonic === "") {
        wallet = await DirectSecp256k1HdWallet.generate();
        mnemonic = wallet.mnemonic;
        await save(COSMOS_MNEMONIC_KEY, wallet.mnemonic);
        console.debug(`wallet not found, created and stored mnemonic starting with ${wallet.mnemonic.split(" ", 4)}`)
    } else {
        wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic);
        console.debug(`retrieved mnemonic starting with ${wallet.mnemonic.split(" ", 4)}`)
    }

    return wallet;
}

const COSMOS_MNEMONIC_KEY = "cosmos_mnemonic_key"

export default function useMnemonic() {
    const [mnemonic, setMnemonic] = useState("")

    useEffect(() => {
        async function loadWallet() {
            try {
                const wallet = await getOrCreateWallet();
                setMnemonic(wallet.mnemonic);
            } catch (e) {
                console.warn(e);
            }
        }

        loadWallet();
    }, []);

    return mnemonic
}