// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: business_api/v1/business.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package business_api.v1;

@kotlin.jvm.JvmName("-initializelistRequest")
public inline fun listRequest(block: business_api.v1.ListRequestKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.ListRequest =
  business_api.v1.ListRequestKt.Dsl._create(business_api.v1.BusinessOuterClass.ListRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {}
 * ```
 *
 * Protobuf type `business_api.v1.ListRequest`
 */
public object ListRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: business_api.v1.BusinessOuterClass.ListRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: business_api.v1.BusinessOuterClass.ListRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): business_api.v1.BusinessOuterClass.ListRequest = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun business_api.v1.BusinessOuterClass.ListRequest.copy(block: `business_api.v1`.ListRequestKt.Dsl.() -> kotlin.Unit): business_api.v1.BusinessOuterClass.ListRequest =
  `business_api.v1`.ListRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()
