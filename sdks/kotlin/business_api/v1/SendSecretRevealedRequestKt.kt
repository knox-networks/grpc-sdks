// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: business_api/v1/business.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package business_api.v1;

@kotlin.jvm.JvmName("-initializesendSecretRevealedRequest")
public inline fun sendSecretRevealedRequest(block: business_api.v1.SendSecretRevealedRequestKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.SendSecretRevealedRequest =
  business_api.v1.SendSecretRevealedRequestKt.Dsl._create(business_api.v1.BusinessOuterClass.SendSecretRevealedRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "participant": "zParticipantPublicKey",
 * "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81",
 * "secret": "9ca6333ba92ea4aaa3e4c972c4c9301f2931154b73373a25b849fce8fd4e16c9"
 * }
 * ```
 *
 * Protobuf type `business_api.v1.SendSecretRevealedRequest`
 */
public object SendSecretRevealedRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: business_api.v1.BusinessOuterClass.SendSecretRevealedRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: business_api.v1.BusinessOuterClass.SendSecretRevealedRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): business_api.v1.BusinessOuterClass.SendSecretRevealedRequest = _builder.build()

    /**
     * ```
     * Participant Public Key.
     * ```
     *
     * `string participant = 1;`
     */
    public var participant: kotlin.String
      @JvmName("getParticipant")
      get() = _builder.getParticipant()
      @JvmName("setParticipant")
      set(value) {
        _builder.setParticipant(value)
      }
    /**
     * ```
     * Participant Public Key.
     * ```
     *
     * `string participant = 1;`
     */
    public fun clearParticipant() {
      _builder.clearParticipant()
    }

    /**
     * ```
     * Contract ID.
     * ```
     *
     * `string contract_id = 2;`
     */
    public var contractId: kotlin.String
      @JvmName("getContractId")
      get() = _builder.getContractId()
      @JvmName("setContractId")
      set(value) {
        _builder.setContractId(value)
      }
    /**
     * ```
     * Contract ID.
     * ```
     *
     * `string contract_id = 2;`
     */
    public fun clearContractId() {
      _builder.clearContractId()
    }

    /**
     * ```
     * Preimage bytes in hex format
     * ```
     *
     * `string secret = 3;`
     */
    public var secret: kotlin.String
      @JvmName("getSecret")
      get() = _builder.getSecret()
      @JvmName("setSecret")
      set(value) {
        _builder.setSecret(value)
      }
    /**
     * ```
     * Preimage bytes in hex format
     * ```
     *
     * `string secret = 3;`
     */
    public fun clearSecret() {
      _builder.clearSecret()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun business_api.v1.BusinessOuterClass.SendSecretRevealedRequest.copy(block: `business_api.v1`.SendSecretRevealedRequestKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.SendSecretRevealedRequest =
  `business_api.v1`.SendSecretRevealedRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

