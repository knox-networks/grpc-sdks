// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: issuer_api/v1/issuer.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package issuer_api.v1;

@kotlin.jvm.JvmName("-initializegetLimitsRequest")
public inline fun getLimitsRequest(block: issuer_api.v1.GetLimitsRequestKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.GetLimitsRequest =
  issuer_api.v1.GetLimitsRequestKt.Dsl._create(issuer_api.v1.Issuer.GetLimitsRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {}
 * ```
 *
 * Protobuf type `issuer_api.v1.GetLimitsRequest`
 */
public object GetLimitsRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: issuer_api.v1.Issuer.GetLimitsRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: issuer_api.v1.Issuer.GetLimitsRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): issuer_api.v1.Issuer.GetLimitsRequest = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun issuer_api.v1.Issuer.GetLimitsRequest.copy(block: `issuer_api.v1`.GetLimitsRequestKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.GetLimitsRequest =
  `issuer_api.v1`.GetLimitsRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

