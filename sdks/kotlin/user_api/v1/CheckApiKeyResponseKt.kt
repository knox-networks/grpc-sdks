// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializecheckApiKeyResponse")
public inline fun checkApiKeyResponse(block: user_api.v1.CheckApiKeyResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.CheckApiKeyResponse =
  user_api.v1.CheckApiKeyResponseKt.Dsl._create(user_api.v1.UserOuterClass.CheckApiKeyResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "is_valid": true
 * }
 * ```
 *
 * Protobuf type `user_api.v1.CheckApiKeyResponse`
 */
public object CheckApiKeyResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.CheckApiKeyResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.CheckApiKeyResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.CheckApiKeyResponse = _builder.build()

    /**
     * ```
     * Returns the validity of the API key.
     * ```
     *
     * `bool is_valid = 3;`
     */
    public var isValid: kotlin.Boolean
      @JvmName("getIsValid")
      get() = _builder.getIsValid()
      @JvmName("setIsValid")
      set(value) {
        _builder.setIsValid(value)
      }
    /**
     * ```
     * Returns the validity of the API key.
     * ```
     *
     * `bool is_valid = 3;`
     */
    public fun clearIsValid() {
      _builder.clearIsValid()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.CheckApiKeyResponse.copy(block: `user_api.v1`.CheckApiKeyResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.CheckApiKeyResponse =
  `user_api.v1`.CheckApiKeyResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

