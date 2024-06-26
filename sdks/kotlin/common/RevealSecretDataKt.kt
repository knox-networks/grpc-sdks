// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/amqp.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializerevealSecretData")
public inline fun revealSecretData(block: common.RevealSecretDataKt.Dsl.() -> kotlin.Unit): common.Amqp.RevealSecretData =
  common.RevealSecretDataKt.Dsl._create(common.Amqp.RevealSecretData.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "participant": "zParticipantPublicKey",
 * "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81"
 * }
 * ```
 *
 * Protobuf type `common.RevealSecretData`
 */
public object RevealSecretDataKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.Amqp.RevealSecretData.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.Amqp.RevealSecretData.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.Amqp.RevealSecretData = _builder.build()

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
public inline fun common.Amqp.RevealSecretData.copy(block: `common`.RevealSecretDataKt.Dsl.() -> kotlin.Unit): common.Amqp.RevealSecretData =
  `common`.RevealSecretDataKt.Dsl._create(this.toBuilder()).apply { block() }._build()

