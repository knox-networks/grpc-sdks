// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: issuer_api/v1/issuer.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package issuer_api.v1;

@kotlin.jvm.JvmName("-initializesetRoleResponse")
public inline fun setRoleResponse(block: issuer_api.v1.SetRoleResponseKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.SetRoleResponse =
  issuer_api.v1.SetRoleResponseKt.Dsl._create(issuer_api.v1.Issuer.SetRoleResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {}
 * ```
 *
 * Protobuf type `issuer_api.v1.SetRoleResponse`
 */
public object SetRoleResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: issuer_api.v1.Issuer.SetRoleResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: issuer_api.v1.Issuer.SetRoleResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): issuer_api.v1.Issuer.SetRoleResponse = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun issuer_api.v1.Issuer.SetRoleResponse.copy(block: `issuer_api.v1`.SetRoleResponseKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.SetRoleResponse =
  `issuer_api.v1`.SetRoleResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()
