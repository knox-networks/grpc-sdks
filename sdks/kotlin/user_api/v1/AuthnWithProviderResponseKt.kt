// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializeauthnWithProviderResponse")
public inline fun authnWithProviderResponse(block: user_api.v1.AuthnWithProviderResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.AuthnWithProviderResponse =
  user_api.v1.AuthnWithProviderResponseKt.Dsl._create(user_api.v1.UserOuterClass.AuthnWithProviderResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "provider_url": "PROVIDER_URL"
 * } 
 * ```
 *
 * Protobuf type `user_api.v1.AuthnWithProviderResponse`
 */
public object AuthnWithProviderResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.AuthnWithProviderResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.AuthnWithProviderResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.AuthnWithProviderResponse = _builder.build()

    /**
     * ```
     * Callback URL to be used for authentication.
     * ```
     *
     * `string provider_url = 1;`
     */
    public var providerUrl: kotlin.String
      @JvmName("getProviderUrl")
      get() = _builder.getProviderUrl()
      @JvmName("setProviderUrl")
      set(value) {
        _builder.setProviderUrl(value)
      }
    /**
     * ```
     * Callback URL to be used for authentication.
     * ```
     *
     * `string provider_url = 1;`
     */
    public fun clearProviderUrl() {
      _builder.clearProviderUrl()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.AuthnWithProviderResponse.copy(block: `user_api.v1`.AuthnWithProviderResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.AuthnWithProviderResponse =
  `user_api.v1`.AuthnWithProviderResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

