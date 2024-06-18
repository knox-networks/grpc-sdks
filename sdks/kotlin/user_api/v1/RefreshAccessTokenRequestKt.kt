// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializerefreshAccessTokenRequest")
public inline fun refreshAccessTokenRequest(block: user_api.v1.RefreshAccessTokenRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.RefreshAccessTokenRequest =
  user_api.v1.RefreshAccessTokenRequestKt.Dsl._create(user_api.v1.UserOuterClass.RefreshAccessTokenRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "refresh_token": "REFRESH_TOKEN"
 * }
 * ```
 *
 * Protobuf type `user_api.v1.RefreshAccessTokenRequest`
 */
public object RefreshAccessTokenRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.RefreshAccessTokenRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.RefreshAccessTokenRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.RefreshAccessTokenRequest = _builder.build()

    /**
     * ```
     * OAuth2 RFC6749 Refresh Token.
     * ```
     *
     * `string refresh_token = 1;`
     */
    public var refreshToken: kotlin.String
      @JvmName("getRefreshToken")
      get() = _builder.getRefreshToken()
      @JvmName("setRefreshToken")
      set(value) {
        _builder.setRefreshToken(value)
      }
    /**
     * ```
     * OAuth2 RFC6749 Refresh Token.
     * ```
     *
     * `string refresh_token = 1;`
     */
    public fun clearRefreshToken() {
      _builder.clearRefreshToken()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.RefreshAccessTokenRequest.copy(block: `user_api.v1`.RefreshAccessTokenRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.RefreshAccessTokenRequest =
  `user_api.v1`.RefreshAccessTokenRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

