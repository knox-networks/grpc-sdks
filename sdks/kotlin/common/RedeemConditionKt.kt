// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/packet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializeredeemCondition")
public inline fun redeemCondition(block: common.RedeemConditionKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.RedeemCondition =
  common.RedeemConditionKt.Dsl._create(common.PacketOuterClass.RedeemCondition.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "issuer": "zIssuerPublicKey"
 * }
 * ```
 *
 * Protobuf type `common.RedeemCondition`
 */
public object RedeemConditionKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.PacketOuterClass.RedeemCondition.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.PacketOuterClass.RedeemCondition.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.PacketOuterClass.RedeemCondition = _builder.build()

    /**
     * ```
     * Public key of the issuer
     * ```
     *
     * `string issuer = 1;`
     */
    public var issuer: kotlin.String
      @JvmName("getIssuer")
      get() = _builder.getIssuer()
      @JvmName("setIssuer")
      set(value) {
        _builder.setIssuer(value)
      }
    /**
     * ```
     * Public key of the issuer
     * ```
     *
     * `string issuer = 1;`
     */
    public fun clearIssuer() {
      _builder.clearIssuer()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.PacketOuterClass.RedeemCondition.copy(block: `common`.RedeemConditionKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.RedeemCondition =
  `common`.RedeemConditionKt.Dsl._create(this.toBuilder()).apply { block() }._build()

