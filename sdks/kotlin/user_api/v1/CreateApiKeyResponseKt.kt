// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializecreateApiKeyResponse")
public inline fun createApiKeyResponse(block: user_api.v1.CreateApiKeyResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.CreateApiKeyResponse =
  user_api.v1.CreateApiKeyResponseKt.Dsl._create(user_api.v1.UserOuterClass.CreateApiKeyResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "api_key_entity": {
 * "client_id": "CLIENT_ID",
 * "client_name": "CLIENT_NAME",
 * "api_key": "API_KEY",
 * "status": 0,
 * "expiry_date": { 
 * "seconds": 0, 
 * "nanos": 0 
 * }
 * }
 * }
 * ```
 *
 * Protobuf type `user_api.v1.CreateApiKeyResponse`
 */
public object CreateApiKeyResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.CreateApiKeyResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.CreateApiKeyResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.CreateApiKeyResponse = _builder.build()

    /**
     * ```
     * API Key Entity.
     * ```
     *
     * `.user_api.v1.ApiKeyEntity api_key_entity = 1;`
     */
    public var apiKeyEntity: user_api.v1.UserOuterClass.ApiKeyEntity
      @JvmName("getApiKeyEntity")
      get() = _builder.getApiKeyEntity()
      @JvmName("setApiKeyEntity")
      set(value) {
        _builder.setApiKeyEntity(value)
      }
    /**
     * ```
     * API Key Entity.
     * ```
     *
     * `.user_api.v1.ApiKeyEntity api_key_entity = 1;`
     */
    public fun clearApiKeyEntity() {
      _builder.clearApiKeyEntity()
    }
    /**
     * ```
     * API Key Entity.
     * ```
     *
     * `.user_api.v1.ApiKeyEntity api_key_entity = 1;`
     * @return Whether the apiKeyEntity field is set.
     */
    public fun hasApiKeyEntity(): kotlin.Boolean {
      return _builder.hasApiKeyEntity()
    }
    public val CreateApiKeyResponseKt.Dsl.apiKeyEntityOrNull: user_api.v1.UserOuterClass.ApiKeyEntity?
      get() = _builder.apiKeyEntityOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.CreateApiKeyResponse.copy(block: `user_api.v1`.CreateApiKeyResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.CreateApiKeyResponse =
  `user_api.v1`.CreateApiKeyResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val user_api.v1.UserOuterClass.CreateApiKeyResponseOrBuilder.apiKeyEntityOrNull: user_api.v1.UserOuterClass.ApiKeyEntity?
  get() = if (hasApiKeyEntity()) getApiKeyEntity() else null

