// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializecreateApiKeyRequest")
public inline fun createApiKeyRequest(block: user_api.v1.CreateApiKeyRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.CreateApiKeyRequest =
  user_api.v1.CreateApiKeyRequestKt.Dsl._create(user_api.v1.UserOuterClass.CreateApiKeyRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "client_name": "CLIENT_NAME"
 * }
 * ```
 *
 * Protobuf type `user_api.v1.CreateApiKeyRequest`
 */
public object CreateApiKeyRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.CreateApiKeyRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.CreateApiKeyRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.CreateApiKeyRequest = _builder.build()

    /**
     * ```
     * Client Name.
     * ```
     *
     * `string client_name = 1;`
     */
    public var clientName: kotlin.String
      @JvmName("getClientName")
      get() = _builder.getClientName()
      @JvmName("setClientName")
      set(value) {
        _builder.setClientName(value)
      }
    /**
     * ```
     * Client Name.
     * ```
     *
     * `string client_name = 1;`
     */
    public fun clearClientName() {
      _builder.clearClientName()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.CreateApiKeyRequest.copy(block: `user_api.v1`.CreateApiKeyRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.CreateApiKeyRequest =
  `user_api.v1`.CreateApiKeyRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()
