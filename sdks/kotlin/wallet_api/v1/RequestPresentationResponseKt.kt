// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializerequestPresentationResponse")
public inline fun requestPresentationResponse(block: wallet_api.v1.RequestPresentationResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.RequestPresentationResponse =
  wallet_api.v1.RequestPresentationResponseKt.Dsl._create(wallet_api.v1.WalletOuterClass.RequestPresentationResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "presentation": "PRESENTATION"
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.RequestPresentationResponse`
 */
public object RequestPresentationResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.RequestPresentationResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.RequestPresentationResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.RequestPresentationResponse = _builder.build()

    /**
     * ```
     * The JSON Verifiable Presentation containing the PII of the subject.
     * ```
     *
     * `string presentation = 1;`
     */
    public var presentation: kotlin.String
      @JvmName("getPresentation")
      get() = _builder.getPresentation()
      @JvmName("setPresentation")
      set(value) {
        _builder.setPresentation(value)
      }
    /**
     * ```
     * The JSON Verifiable Presentation containing the PII of the subject.
     * ```
     *
     * `string presentation = 1;`
     */
    public fun clearPresentation() {
      _builder.clearPresentation()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.RequestPresentationResponse.copy(block: `wallet_api.v1`.RequestPresentationResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.RequestPresentationResponse =
  `wallet_api.v1`.RequestPresentationResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()
