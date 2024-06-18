// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/packet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializesimplePaymentRequest")
public inline fun simplePaymentRequest(block: common.SimplePaymentRequestKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.SimplePaymentRequest =
  common.SimplePaymentRequestKt.Dsl._create(common.PacketOuterClass.SimplePaymentRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "amount": {
 * "currency_code":"USD",
 * "amount":100,
 * "decimals":2
 * }
 * }
 * ```
 *
 * Protobuf type `common.SimplePaymentRequest`
 */
public object SimplePaymentRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.PacketOuterClass.SimplePaymentRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.PacketOuterClass.SimplePaymentRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.PacketOuterClass.SimplePaymentRequest = _builder.build()

    /**
     * ```
     * Amount in simple payment.
     * ```
     *
     * `.common.Amount amount = 1;`
     */
    public var amount: common.Common.Amount
      @JvmName("getAmount")
      get() = _builder.getAmount()
      @JvmName("setAmount")
      set(value) {
        _builder.setAmount(value)
      }
    /**
     * ```
     * Amount in simple payment.
     * ```
     *
     * `.common.Amount amount = 1;`
     */
    public fun clearAmount() {
      _builder.clearAmount()
    }
    /**
     * ```
     * Amount in simple payment.
     * ```
     *
     * `.common.Amount amount = 1;`
     * @return Whether the amount field is set.
     */
    public fun hasAmount(): kotlin.Boolean {
      return _builder.hasAmount()
    }
    public val SimplePaymentRequestKt.Dsl.amountOrNull: common.Common.Amount?
      get() = _builder.amountOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.PacketOuterClass.SimplePaymentRequest.copy(block: `common`.SimplePaymentRequestKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.SimplePaymentRequest =
  `common`.SimplePaymentRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val common.PacketOuterClass.SimplePaymentRequestOrBuilder.amountOrNull: common.Common.Amount?
  get() = if (hasAmount()) getAmount() else null

