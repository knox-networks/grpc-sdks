// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializegetApiKeysRequest")
public inline fun getApiKeysRequest(block: user_api.v1.GetApiKeysRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.GetApiKeysRequest =
  user_api.v1.GetApiKeysRequestKt.Dsl._create(user_api.v1.UserOuterClass.GetApiKeysRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {}
 * ```
 *
 * Protobuf type `user_api.v1.GetApiKeysRequest`
 */
public object GetApiKeysRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.GetApiKeysRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.GetApiKeysRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.GetApiKeysRequest = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.GetApiKeysRequest.copy(block: `user_api.v1`.GetApiKeysRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.GetApiKeysRequest =
  `user_api.v1`.GetApiKeysRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

