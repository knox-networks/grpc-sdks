// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializecreateOwnerResponse")
public inline fun createOwnerResponse(block: wallet_api.v1.CreateOwnerResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.CreateOwnerResponse =
  wallet_api.v1.CreateOwnerResponseKt.Dsl._create(wallet_api.v1.WalletOuterClass.CreateOwnerResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "owner_id": "eyJwtHeader.eyJwtPayload.Secret",
 * "api_key": "API_KEY",
 * "owner_name": "OWNER_NAME"
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.CreateOwnerResponse`
 */
public object CreateOwnerResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.CreateOwnerResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.CreateOwnerResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.CreateOwnerResponse = _builder.build()

    /**
     * ```
     * UUID of the created owner of the wallet(s).
     * ```
     *
     * `string owner_id = 1;`
     */
    public var ownerId: kotlin.String
      @JvmName("getOwnerId")
      get() = _builder.getOwnerId()
      @JvmName("setOwnerId")
      set(value) {
        _builder.setOwnerId(value)
      }
    /**
     * ```
     * UUID of the created owner of the wallet(s).
     * ```
     *
     * `string owner_id = 1;`
     */
    public fun clearOwnerId() {
      _builder.clearOwnerId()
    }

    /**
     * ```
     * API key given to the wallet owner as part of creation.
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
     * API key given to the wallet owner as part of creation.
     * ```
     *
     * `string api_key = 2;`
     */
    public fun clearApiKey() {
      _builder.clearApiKey()
    }

    /**
     * ```
     * Name of the wallet owner.
     * ```
     *
     * `string owner_name = 3;`
     */
    public var ownerName: kotlin.String
      @JvmName("getOwnerName")
      get() = _builder.getOwnerName()
      @JvmName("setOwnerName")
      set(value) {
        _builder.setOwnerName(value)
      }
    /**
     * ```
     * Name of the wallet owner.
     * ```
     *
     * `string owner_name = 3;`
     */
    public fun clearOwnerName() {
      _builder.clearOwnerName()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.CreateOwnerResponse.copy(block: `wallet_api.v1`.CreateOwnerResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.CreateOwnerResponse =
  `wallet_api.v1`.CreateOwnerResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

