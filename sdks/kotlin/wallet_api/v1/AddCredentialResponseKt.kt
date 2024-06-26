// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializeaddCredentialResponse")
public inline fun addCredentialResponse(block: wallet_api.v1.AddCredentialResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.AddCredentialResponse =
  wallet_api.v1.AddCredentialResponseKt.Dsl._create(wallet_api.v1.WalletOuterClass.AddCredentialResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "credential": "CREDENTIAL"
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.AddCredentialResponse`
 */
public object AddCredentialResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.AddCredentialResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.AddCredentialResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.AddCredentialResponse = _builder.build()

    /**
     * ```
     * W3C Verifiable Credential representing the subject.
     * ```
     *
     * `string credential = 1;`
     */
    public var credential: kotlin.String
      @JvmName("getCredential")
      get() = _builder.getCredential()
      @JvmName("setCredential")
      set(value) {
        _builder.setCredential(value)
      }
    /**
     * ```
     * W3C Verifiable Credential representing the subject.
     * ```
     *
     * `string credential = 1;`
     */
    public fun clearCredential() {
      _builder.clearCredential()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.AddCredentialResponse.copy(block: `wallet_api.v1`.AddCredentialResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.AddCredentialResponse =
  `wallet_api.v1`.AddCredentialResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

