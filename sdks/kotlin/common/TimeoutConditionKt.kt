// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/packet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializetimeoutCondition")
public inline fun timeoutCondition(block: common.TimeoutConditionKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.TimeoutCondition =
  common.TimeoutConditionKt.Dsl._create(common.PacketOuterClass.TimeoutCondition.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "timeout": 1687180000
 * }
 * ```
 *
 * Protobuf type `common.TimeoutCondition`
 */
public object TimeoutConditionKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.PacketOuterClass.TimeoutCondition.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.PacketOuterClass.TimeoutCondition.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.PacketOuterClass.TimeoutCondition = _builder.build()

    /**
     * ```
     * Unix timestamp of timeout datetime.
     * ```
     *
     * `uint64 timeout = 1;`
     */
    public var timeout: kotlin.Long
      @JvmName("getTimeout")
      get() = _builder.getTimeout()
      @JvmName("setTimeout")
      set(value) {
        _builder.setTimeout(value)
      }
    /**
     * ```
     * Unix timestamp of timeout datetime.
     * ```
     *
     * `uint64 timeout = 1;`
     */
    public fun clearTimeout() {
      _builder.clearTimeout()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.PacketOuterClass.TimeoutCondition.copy(block: `common`.TimeoutConditionKt.Dsl.() -> kotlin.Unit): common.PacketOuterClass.TimeoutCondition =
  `common`.TimeoutConditionKt.Dsl._create(this.toBuilder()).apply { block() }._build()
