// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializeresumePausedTransactionRequest")
public inline fun resumePausedTransactionRequest(block: wallet_api.v1.ResumePausedTransactionRequestKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ResumePausedTransactionRequest =
  wallet_api.v1.ResumePausedTransactionRequestKt.Dsl._create(wallet_api.v1.WalletOuterClass.ResumePausedTransactionRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "wallet_id": "zWalletIDMultibase58Encoded",
 * "api_key": "API_KEY",
 * "uetr": "UETR",
 * "accepted": true
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.ResumePausedTransactionRequest`
 */
public object ResumePausedTransactionRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.ResumePausedTransactionRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.ResumePausedTransactionRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.ResumePausedTransactionRequest = _builder.build()

    /**
     * ```
     * UUID of the wallet.
     * ```
     *
     * `string wallet_id = 1;`
     */
    public var walletId: kotlin.String
      @JvmName("getWalletId")
      get() = _builder.getWalletId()
      @JvmName("setWalletId")
      set(value) {
        _builder.setWalletId(value)
      }
    /**
     * ```
     * UUID of the wallet.
     * ```
     *
     * `string wallet_id = 1;`
     */
    public fun clearWalletId() {
      _builder.clearWalletId()
    }

    /**
     * ```
     * API key given to the wallet owner at the time of owner creation.
     * ```
     *
     * `string api_key = 2;`
     */
    public var apiKey: kotlin.String
      @JvmName("getApiKey")
      get() = _builder.getApiKey()
      @JvmName("setApiKey")
      set(value) {
        _builder.setApiKey(value)
      }
    /**
     * ```
     * API key given to the wallet owner at the time of owner creation.
     * ```
     *
     * `string api_key = 2;`
     */
    public fun clearApiKey() {
      _builder.clearApiKey()
    }

    /**
     * ```
     * A Unique End-to-end Transaction Reference, string of 36 unique characters.
     * ```
     *
     * `string uetr = 3;`
     */
    public var uetr: kotlin.String
      @JvmName("getUetr")
      get() = _builder.getUetr()
      @JvmName("setUetr")
      set(value) {
        _builder.setUetr(value)
      }
    /**
     * ```
     * A Unique End-to-end Transaction Reference, string of 36 unique characters.
     * ```
     *
     * `string uetr = 3;`
     */
    public fun clearUetr() {
      _builder.clearUetr()
    }

    /**
     * ```
     * Bool to indicate whether to accept or reject the transaction.
     * ```
     *
     * `bool accept = 4;`
     */
    public var accept: kotlin.Boolean
      @JvmName("getAccept")
      get() = _builder.getAccept()
      @JvmName("setAccept")
      set(value) {
        _builder.setAccept(value)
      }
    /**
     * ```
     * Bool to indicate whether to accept or reject the transaction.
     * ```
     *
     * `bool accept = 4;`
     */
    public fun clearAccept() {
      _builder.clearAccept()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.ResumePausedTransactionRequest.copy(block: `wallet_api.v1`.ResumePausedTransactionRequestKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ResumePausedTransactionRequest =
  `wallet_api.v1`.ResumePausedTransactionRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

