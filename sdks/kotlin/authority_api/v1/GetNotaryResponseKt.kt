// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: authority_api/v1/authority.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package authority_api.v1;

@kotlin.jvm.JvmName("-initializegetNotaryResponse")
public inline fun getNotaryResponse(block: authority_api.v1.GetNotaryResponseKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.GetNotaryResponse =
  authority_api.v1.GetNotaryResponseKt.Dsl._create(authority_api.v1.AuthorityOuterClass.GetNotaryResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "signature": "ByteLengthSignatureVariesByAlgo="
 * }
 * ```
 *
 * Protobuf type `authority_api.v1.GetNotaryResponse`
 */
public object GetNotaryResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: authority_api.v1.AuthorityOuterClass.GetNotaryResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: authority_api.v1.AuthorityOuterClass.GetNotaryResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): authority_api.v1.AuthorityOuterClass.GetNotaryResponse = _builder.build()

    /**
     * ```
     * The signature of the Notary created by signing the Notary Public key with the Authority keypair
     * ```
     *
     * `bytes signature = 1;`
     */
    public var signature: com.google.protobuf.ByteString
      @JvmName("getSignature")
      get() = _builder.getSignature()
      @JvmName("setSignature")
      set(value) {
        _builder.setSignature(value)
      }
    /**
     * ```
     * The signature of the Notary created by signing the Notary Public key with the Authority keypair
     * ```
     *
     * `bytes signature = 1;`
     */
    public fun clearSignature() {
      _builder.clearSignature()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun authority_api.v1.AuthorityOuterClass.GetNotaryResponse.copy(block: `authority_api.v1`.GetNotaryResponseKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.GetNotaryResponse =
  `authority_api.v1`.GetNotaryResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

