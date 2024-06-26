// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: business_api/v1/business.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package business_api.v1;

@kotlin.jvm.JvmName("-initializesendProposalNackedRequest")
public inline fun sendProposalNackedRequest(block: business_api.v1.SendProposalNackedRequestKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.SendProposalNackedRequest =
  business_api.v1.SendProposalNackedRequestKt.Dsl._create(business_api.v1.BusinessOuterClass.SendProposalNackedRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "participant": "zParticipantPublicKey",
 * "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81"
 * }
 * ```
 *
 * Protobuf type `business_api.v1.SendProposalNackedRequest`
 */
public object SendProposalNackedRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: business_api.v1.BusinessOuterClass.SendProposalNackedRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: business_api.v1.BusinessOuterClass.SendProposalNackedRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): business_api.v1.BusinessOuterClass.SendProposalNackedRequest = _builder.build()

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
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun business_api.v1.BusinessOuterClass.SendProposalNackedRequest.copy(block: `business_api.v1`.SendProposalNackedRequestKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.SendProposalNackedRequest =
  `business_api.v1`.SendProposalNackedRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

