// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: authority_api/v1/authority.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package authority_api.v1;

@kotlin.jvm.JvmName("-initializeremoveAssetResponse")
public inline fun removeAssetResponse(block: authority_api.v1.RemoveAssetResponseKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.RemoveAssetResponse =
  authority_api.v1.RemoveAssetResponseKt.Dsl._create(authority_api.v1.AuthorityOuterClass.RemoveAssetResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {}
 * ```
 *
 * Protobuf type `authority_api.v1.RemoveAssetResponse`
 */
public object RemoveAssetResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: authority_api.v1.AuthorityOuterClass.RemoveAssetResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: authority_api.v1.AuthorityOuterClass.RemoveAssetResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): authority_api.v1.AuthorityOuterClass.RemoveAssetResponse = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun authority_api.v1.AuthorityOuterClass.RemoveAssetResponse.copy(block: `authority_api.v1`.RemoveAssetResponseKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.RemoveAssetResponse =
  `authority_api.v1`.RemoveAssetResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

