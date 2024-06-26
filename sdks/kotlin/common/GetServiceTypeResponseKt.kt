// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: common/v1/common.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package common;

@kotlin.jvm.JvmName("-initializegetServiceTypeResponse")
public inline fun getServiceTypeResponse(block: common.GetServiceTypeResponseKt.Dsl.() -> kotlin.Unit): common.Common.GetServiceTypeResponse =
  common.GetServiceTypeResponseKt.Dsl._create(common.Common.GetServiceTypeResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "service_type": 1
 * }
 * ```
 *
 * Protobuf type `common.GetServiceTypeResponse`
 */
public object GetServiceTypeResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: common.Common.GetServiceTypeResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: common.Common.GetServiceTypeResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): common.Common.GetServiceTypeResponse = _builder.build()

    /**
     * ```
     * One of the service types defined in ServiceType.
     * ```
     *
     * `.common.ServiceType service_type = 1;`
     */
    public var serviceType: common.Common.ServiceType
      @JvmName("getServiceType")
      get() = _builder.getServiceType()
      @JvmName("setServiceType")
      set(value) {
        _builder.setServiceType(value)
      }
    public var serviceTypeValue: kotlin.Int
      @JvmName("getServiceTypeValue")
      get() = _builder.getServiceTypeValue()
      @JvmName("setServiceTypeValue")
      set(value) {
        _builder.setServiceTypeValue(value)
      }
    /**
     * ```
     * One of the service types defined in ServiceType.
     * ```
     *
     * `.common.ServiceType service_type = 1;`
     */
    public fun clearServiceType() {
      _builder.clearServiceType()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun common.Common.GetServiceTypeResponse.copy(block: `common`.GetServiceTypeResponseKt.Dsl.() -> kotlin.Unit): common.Common.GetServiceTypeResponse =
  `common`.GetServiceTypeResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

