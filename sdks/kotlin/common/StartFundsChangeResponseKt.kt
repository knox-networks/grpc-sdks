// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/common.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializestartFundsChangeResponse")
public inline fun startFundsChangeResponse(block: common.StartFundsChangeResponseKt.Dsl.() -> kotlin.Unit): common.Common.StartFundsChangeResponse =
  common.StartFundsChangeResponseKt.Dsl._create(common.Common.StartFundsChangeResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "uetr": "UETR"
 * }
 * ```
 *
 * Protobuf type `common.StartFundsChangeResponse`
 */
public object StartFundsChangeResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.Common.StartFundsChangeResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.Common.StartFundsChangeResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.Common.StartFundsChangeResponse = _builder.build()

    /**
     * ```
     * Universal E2E Transaction Reference (UUID v4 format).
     * ```
     *
     * `string uetr = 1;`
     */
    public var uetr: kotlin.String
      @JvmName("getUetr")
      get() = _builder.getUetr()
      @JvmName("setUetr")
      set(value) {
        _builder.setUetr(value)
      }
    /**
     * ```
     * Universal E2E Transaction Reference (UUID v4 format).
     * ```
     *
     * `string uetr = 1;`
     */
    public fun clearUetr() {
      _builder.clearUetr()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.Common.StartFundsChangeResponse.copy(block: `common`.StartFundsChangeResponseKt.Dsl.() -> kotlin.Unit): common.Common.StartFundsChangeResponse =
  `common`.StartFundsChangeResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

