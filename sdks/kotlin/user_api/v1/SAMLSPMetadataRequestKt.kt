// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializesAMLSPMetadataRequest")
public inline fun sAMLSPMetadataRequest(block: user_api.v1.SAMLSPMetadataRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.SAMLSPMetadataRequest =
  user_api.v1.SAMLSPMetadataRequestKt.Dsl._create(user_api.v1.UserOuterClass.SAMLSPMetadataRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {} 
 * ```
 *
 * Protobuf type `user_api.v1.SAMLSPMetadataRequest`
 */
public object SAMLSPMetadataRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.SAMLSPMetadataRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.SAMLSPMetadataRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.SAMLSPMetadataRequest = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.SAMLSPMetadataRequest.copy(block: `user_api.v1`.SAMLSPMetadataRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.SAMLSPMetadataRequest =
  `user_api.v1`.SAMLSPMetadataRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

