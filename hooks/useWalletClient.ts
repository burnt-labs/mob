import {useEffect, useState} from 'react';
import * as SecureStore from "expo-secure-store";
import { Tendermint34Client } from "@cosmjs/tendermint-rpc";
import {QueryClient, setupAuthzExtension, SigningStargateClient} from '@cosmjs/stargate';
import {DirectSecp256k1HdWallet, AccountData} from "@cosmjs/proto-signing";
import { Grant } from 'cosmjs-types/cosmos/authz/v1beta1/authz';
import {DirectSecp256k1HdWalletOptions} from "@cosmjs/proto-signing/build/directsecp256k1hdwallet";
import {COSMOS_MNEMONIC_KEY, TENDERMINT_RPC} from "../constants/Cosmos";
// import {COSMOS_MNEMONIC_KEY} from "constants/Cosmos"

async function save(key: string, value: string) {
    await SecureStore.setItemAsync(key, value);
}

async function getValueFor(key: string) {
    const value = await SecureStore.getItemAsync(key);
    return value || "";
}

async function getWalletClient() {
    const wallet = await getOrCreateWallet();
    return await SigningStargateClient.connectWithSigner(TENDERMINT_RPC, wallet);
}

async function getOrCreateWallet() {
    let mnemonic = await getValueFor(COSMOS_MNEMONIC_KEY);

    const options: DirectSecp256k1HdWalletOptions = {
        prefix: "burnt"
    }

    let wallet: DirectSecp256k1HdWallet;
    if (mnemonic === "") {
        console.log("generating wallet");
        wallet = await DirectSecp256k1HdWallet.generate(24, options);
        console.log("generated wallet");
        await save(COSMOS_MNEMONIC_KEY, wallet.mnemonic);
        console.log("saved mnemonic");
        console.debug(`wallet not found, created and stored mnemonic starting with ${wallet.mnemonic.split(" ", 4)}`)
    } else {
        wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, options);
        console.debug(`retrieved mnemonic starting with ${wallet.mnemonic.split(" ", 4)}`)
    }

    return wallet;
}


interface WalletInfo {
    mnemonic: string;
    accounts: readonly AccountData[];
    grants: Grant[];
    authURL: URL;
}

export default function useWalletInfo() {
    const [walletInfo, setWalletInfo] = useState<WalletInfo | undefined>();

    useEffect(() => {
        async function loadWallet() {
            try {
                const wallet = await getOrCreateWallet();
                const accounts = await wallet.getAccounts();
                const grants = await queryAuthzGranteeGrants();
                // const grants = [] as Grant[];

                let authURL = new URL("burnt.com/dashboard");
                authURL.searchParams.append("delegatePublicKey", accounts[0].address);
                authURL.searchParams.append("appContract", "testContractAddress");


                setWalletInfo({
                    mnemonic: wallet.mnemonic,
                    accounts,
                    grants,
                    authURL,
                });
            } catch (e) {
                console.warn(e);
            }
        }

        loadWallet();
    }, []);

    return walletInfo;
}

export async function getAccount() {
    const wallet = await getOrCreateWallet();
    const accounts = await wallet.getAccounts();
    return accounts[0];
}

async function queryAuthzGranteeGrants() {
    console.log("getting tm client");
    const tmClient = await Tendermint34Client.connect(TENDERMINT_RPC);
    console.log("getting query client");
    let queryClient = QueryClient.withExtensions(tmClient, setupAuthzExtension);

    // const granter = await getValueFor(COSMOS_GRANTING_ADDRESS_KEY);
    const account = await getAccount();
    console.log("got account: ", account)

    let grants = [] as Grant[];
    // console.log("getting grants");
    let response = await queryClient.authz.granteeGrants(account.address);
    console.log("gor response: ", response);
    grants = response.grants;
    while (response.pagination && response.pagination.nextKey.length > 0) {
        response = await queryClient.authz.granteeGrants(account.address, response.pagination.nextKey);
        grants.push(...response.grants);
    }
    console.log("got grants:", grants);
    tmClient.disconnect();

    return grants;
}