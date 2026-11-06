package com.burnt.mob

import expo.modules.kotlin.modules.Module
import expo.modules.kotlin.modules.ModuleDefinition
import uniffi.mob.Client
import uniffi.mob.ChainConfig
import uniffi.mob.Coin
import uniffi.mob.Message
import uniffi.mob.MobSessionManager
import uniffi.mob.RustSigner
import uniffi.mob.SessionMetadata
import com.burnt.mob.NativeHttpTransport

class MobModule : Module() {
    // In-memory registries
    private val signers = mutableMapOf<String, RustSigner>()
    private val clients = mutableMapOf<String, Client>()
    private val sessionManagers = mutableMapOf<String, MobSessionManager>()
    private val transport = NativeHttpTransport()
    private var clientCounter = 0
    private var sessionCounter = 0

    override fun definition() = ModuleDefinition {
        Name("Mob")

        // MARK: Signer Management

        AsyncFunction("createSignerFromMnemonic") { mnemonic: String, prefix: String, derivationPath: String? ->
            val signer = RustSigner.fromMnemonic(mnemonic, prefix, derivationPath)
            val address = signer.address()
            val pubKeyHex = signer.publicKeyHex()
            signers[address] = signer
            mapOf("address" to address, "publicKeyHex" to pubKeyHex)
        }

        AsyncFunction("signBytes") { signerAddress: String, message: List<Int> ->
            val signer = signers[signerAddress]
                ?: throw IllegalArgumentException("Signer not found: $signerAddress")
            signer.signBytes(message.toBytes()).toIntList()
        }

        AsyncFunction("destroySigner") { address: String ->
            signers.remove(address)
            Unit
        }

        // MARK: Client Management

        AsyncFunction("createClient") { config: Map<String, Any?> ->
            val chainConfig = parseChainConfig(config)
            val client = Client(chainConfig, transport)
            val clientId = nextClientId()
            clients[clientId] = client
            clientId
        }

        AsyncFunction("createClientWithSigner") { config: Map<String, Any?>, signerAddress: String ->
            val signer = signers[signerAddress]
                ?: throw IllegalArgumentException("Signer not found: $signerAddress")
            val chainConfig = parseChainConfig(config)
            val client = Client.newWithSigner(chainConfig, signer, transport)
            val clientId = nextClientId()
            clients[clientId] = client
            clientId
        }

        AsyncFunction("destroyClient") { clientId: String ->
            clients.remove(clientId)
            Unit
        }

        // MARK: Read Queries

        AsyncFunction("getAccount") { clientId: String, address: String ->
            val client = requireClient(clientId)
            val info = client.getAccount(address)
            mapOf(
                "address" to info.address,
                "accountNumber" to info.accountNumber.toString(),
                "sequence" to info.sequence.toDouble(),
                "pubKey" to info.pubKey
            )
        }

        AsyncFunction("getBalance") { clientId: String, address: String, denom: String ->
            val client = requireClient(clientId)
            val coin = client.getBalance(address, denom)
            mapOf("denom" to coin.denom, "amount" to coin.amount)
        }

        AsyncFunction("getAllBalances") { clientId: String, address: String ->
            val client = requireClient(clientId)
            client.getAllBalances(address).map {
                mapOf("denom" to it.denom, "amount" to it.amount)
            }
        }

        AsyncFunction("getHeight") { clientId: String ->
            val client = requireClient(clientId)
            client.getHeight()
        }

        AsyncFunction("getTx") { clientId: String, hash: String ->
            val client = requireClient(clientId)
            serializeTxResponse(client.getTx(hash))
        }

        AsyncFunction("hasGrants") { clientId: String, granter: String, grantee: String ->
            val client = requireClient(clientId)
            client.hasGrants(granter, grantee)
        }

        AsyncFunction("queryContractSmart") { clientId: String, contractAddress: String, queryMsg: List<Int> ->
            val client = requireClient(clientId)
            client.queryContractSmart(contractAddress, queryMsg.toBytes()).toIntList()
        }

        // MARK: Transactions

        AsyncFunction("send") { clientId: String, toAddress: String, amount: List<Map<String, String>>, memo: String? ->
            val client = requireClient(clientId)
            val coins = parseCoins(amount)
            serializeTxResponse(client.send(toAddress, coins, memo))
        }

        AsyncFunction("executeContract") { clientId: String, contractAddress: String, msg: List<Int>, funds: List<Map<String, String>>, memo: String?, gasLimit: Long? ->
            val client = requireClient(clientId)
            val fundCoins = parseCoins(funds)
            val gl = gasLimit?.toULong()
            serializeTxResponse(
                client.executeContract(contractAddress, msg.toBytes(), fundCoins, null, null, memo, gl)
            )
        }

        AsyncFunction("signAndBroadcastMulti") { clientId: String, messages: List<Map<String, Any?>>, memo: String?, gasLimit: Long? ->
            val client = requireClient(clientId)
            val msgs = parseMessages(messages)
            val gl = gasLimit?.toULong()
            serializeTxResponse(client.signAndBroadcastMulti(msgs, memo, gl))
        }

        // MARK: Session Manager

        AsyncFunction("createSessionManager") { addressPrefix: String ->
            val mgr = MobSessionManager(addressPrefix)
            val sessionId = nextSessionId()
            sessionManagers[sessionId] = mgr
            sessionId
        }

        AsyncFunction("sessionGenerateKey") { sessionId: String ->
            val mgr = requireSessionManager(sessionId)
            val info = mgr.generateSessionKey()
            mapOf("address" to info.address, "publicKeyHex" to info.publicKeyHex)
        }

        AsyncFunction("sessionActivate") { sessionId: String, granter: String, grantee: String, createdAt: Long, expiresAt: Long, description: String?, config: Map<String, Any?> ->
            val mgr = requireSessionManager(sessionId)
            val metadata = SessionMetadata(
                granter = granter,
                grantee = grantee,
                feeGranter = null,
                feePayer = null,
                createdAt = createdAt.toULong(),
                expiresAt = expiresAt.toULong(),
                description = description
            )
            val chainConfig = parseChainConfig(config)
            mgr.activate(metadata, chainConfig, transport)
        }

        AsyncFunction("sessionExport") { sessionId: String ->
            val mgr = requireSessionManager(sessionId)
            mgr.exportSession().toIntList()
        }

        AsyncFunction("sessionRestore") { data: List<Int>, config: Map<String, Any?> ->
            val chainConfig = parseChainConfig(config)
            val mgr = MobSessionManager.restore(data.toBytes(), chainConfig, transport)
            val sessionId = nextSessionId()
            sessionManagers[sessionId] = mgr
            sessionId
        }

        AsyncFunction("sessionDeactivate") { sessionId: String ->
            sessionManagers[sessionId]?.deactivate()
            sessionManagers.remove(sessionId)
            Unit
        }

        AsyncFunction("sessionIsActive") { sessionId: String ->
            sessionManagers[sessionId]?.isActive() ?: false
        }

        AsyncFunction("sessionGranterAddress") { sessionId: String ->
            sessionManagers[sessionId]?.granterAddress()
        }

        AsyncFunction("sessionGranteeAddress") { sessionId: String ->
            sessionManagers[sessionId]?.granteeAddress()
        }

        AsyncFunction("sessionSignBytes") { sessionId: String, message: List<Int> ->
            val mgr = requireSessionManager(sessionId)
            mgr.signBytes(message.toBytes()).toIntList()
        }

        AsyncFunction("sessionSend") { sessionId: String, toAddress: String, amount: List<Map<String, String>>, memo: String? ->
            val mgr = requireSessionManager(sessionId)
            val client = mgr.client()
            val coins = parseCoins(amount)
            serializeTxResponse(client.send(toAddress, coins, memo))
        }

        AsyncFunction("sessionExecuteContract") { sessionId: String, contractAddress: String, msg: List<Int>, funds: List<Map<String, String>>, memo: String?, gasLimit: Long? ->
            val mgr = requireSessionManager(sessionId)
            val client = mgr.client()
            val fundCoins = parseCoins(funds)
            val gl = gasLimit?.toULong()
            serializeTxResponse(
                client.executeContract(contractAddress, msg.toBytes(), fundCoins, null, null, memo, gl)
            )
        }

        AsyncFunction("sessionSignAndBroadcastMulti") { sessionId: String, messages: List<Map<String, Any?>>, memo: String?, gasLimit: Long? ->
            val mgr = requireSessionManager(sessionId)
            val client = mgr.client()
            val msgs = parseMessages(messages)
            val gl = gasLimit?.toULong()
            serializeTxResponse(client.signAndBroadcastMulti(msgs, memo, gl))
        }

        AsyncFunction("sessionQueryContractSmart") { sessionId: String, contractAddress: String, queryMsg: List<Int> ->
            val mgr = requireSessionManager(sessionId)
            val client = mgr.client()
            client.queryContractSmart(contractAddress, queryMsg.toBytes()).toIntList()
        }
    }

    // MARK: Helpers

    private fun nextClientId(): String {
        clientCounter++
        return "client_$clientCounter"
    }

    private fun nextSessionId(): String {
        sessionCounter++
        return "session_$sessionCounter"
    }

    private fun requireClient(clientId: String): Client {
        return clients[clientId]
            ?: throw IllegalArgumentException("Client not found: $clientId")
    }

    private fun requireSessionManager(sessionId: String): MobSessionManager {
        return sessionManagers[sessionId]
            ?: throw IllegalArgumentException("Session manager not found: $sessionId")
    }

    private fun parseChainConfig(config: Map<String, Any?>): ChainConfig {
        val chainId = config["chainId"] as? String
            ?: throw IllegalArgumentException("Missing chainId")
        val rpcEndpoint = config["rpcEndpoint"] as? String
            ?: throw IllegalArgumentException("Missing rpcEndpoint")
        val addressPrefix = config["addressPrefix"] as? String
            ?: throw IllegalArgumentException("Missing addressPrefix")

        return ChainConfig(
            chainId = chainId,
            rpcEndpoint = rpcEndpoint,
            grpcEndpoint = config["grpcEndpoint"] as? String,
            addressPrefix = addressPrefix,
            coinType = (config["coinType"] as? Number)?.toInt()?.toUInt() ?: 118u,
            gasPrice = config["gasPrice"] as? String ?: "0.025",
            feeGranter = config["feeGranter"] as? String
        )
    }

    private fun parseCoins(dicts: List<Map<String, String>>): List<Coin> {
        return dicts.map { dict ->
            Coin(
                denom = dict["denom"] ?: throw IllegalArgumentException("Coin missing 'denom'"),
                amount = dict["amount"] ?: throw IllegalArgumentException("Coin missing 'amount'")
            )
        }
    }

    @Suppress("UNCHECKED_CAST")
    private fun parseMessages(dicts: List<Map<String, Any?>>): List<Message> {
        return dicts.map { dict ->
            val typeUrl = dict["typeUrl"] as? String
                ?: throw IllegalArgumentException("Message missing 'typeUrl'")
            val value = (dict["value"] as? List<Int>)?.toBytes()
                ?: throw IllegalArgumentException("Message missing 'value'")
            Message(typeUrl = typeUrl, value = value)
        }
    }

    private fun serializeTxResponse(tx: uniffi.mob.TxResponse): Map<String, Any> {
        return mapOf(
            "txhash" to tx.txhash,
            "code" to tx.code,
            "rawLog" to tx.rawLog,
            "gasWanted" to tx.gasWanted,
            "gasUsed" to tx.gasUsed,
            "height" to tx.height
        )
    }
}

// JS passes and receives byte arrays as lists of 0..255 numbers.
private fun List<Int>.toBytes(): ByteArray = ByteArray(size) { this[it].toByte() }

private fun ByteArray.toIntList(): List<Int> = map { it.toInt() and 0xFF }

