// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: issuer_api/v1/issuer.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package issuer_api.v1;

@kotlin.jvm.JvmName("-initializegetAssetAuthoritiesRequest")
public inline fun getAssetAuthoritiesRequest(block: issuer_api.v1.GetAssetAuthoritiesRequestKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.GetAssetAuthoritiesRequest =
  issuer_api.v1.GetAssetAuthoritiesRequestKt.Dsl._create(issuer_api.v1.Issuer.GetAssetAuthoritiesRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {}
 * ```
 *
 * Protobuf type `issuer_api.v1.GetAssetAuthoritiesRequest`
 */
public object GetAssetAuthoritiesRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: issuer_api.v1.Issuer.GetAssetAuthoritiesRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: issuer_api.v1.Issuer.GetAssetAuthoritiesRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): issuer_api.v1.Issuer.GetAssetAuthoritiesRequest = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun issuer_api.v1.Issuer.GetAssetAuthoritiesRequest.copy(block: `issuer_api.v1`.GetAssetAuthoritiesRequestKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.GetAssetAuthoritiesRequest =
  `issuer_api.v1`.GetAssetAuthoritiesRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

