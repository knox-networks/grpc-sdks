// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializecreateOwnerRequest")
public inline fun createOwnerRequest(block: wallet_api.v1.CreateOwnerRequestKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.CreateOwnerRequest =
  wallet_api.v1.CreateOwnerRequestKt.Dsl._create(wallet_api.v1.WalletOuterClass.CreateOwnerRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "owner_name": "OWNER_NAME"
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.CreateOwnerRequest`
 */
public object CreateOwnerRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.CreateOwnerRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.CreateOwnerRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.CreateOwnerRequest = _builder.build()

    /**
     * ```
     * Name of the wallet owner.
     * ```
     *
     * `string owner_name = 1;`
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
     * `string owner_name = 1;`
     */
    public fun clearOwnerName() {
      _builder.clearOwnerName()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.CreateOwnerRequest.copy(block: `wallet_api.v1`.CreateOwnerRequestKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.CreateOwnerRequest =
  `wallet_api.v1`.CreateOwnerRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

