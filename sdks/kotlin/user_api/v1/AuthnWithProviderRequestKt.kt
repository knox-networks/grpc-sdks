// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializeauthnWithProviderRequest")
public inline fun authnWithProviderRequest(block: user_api.v1.AuthnWithProviderRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.AuthnWithProviderRequest =
  user_api.v1.AuthnWithProviderRequestKt.Dsl._create(user_api.v1.UserOuterClass.AuthnWithProviderRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "provider": 0, 
 * "instance_name": "INSTANCE_NAME", 
 * "request_origin": "REQUEST_ORIGIN", 
 * "client_state": { } 
 * }
 * ```
 *
 * Protobuf type `user_api.v1.AuthnWithProviderRequest`
 */
public object AuthnWithProviderRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.AuthnWithProviderRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.AuthnWithProviderRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.AuthnWithProviderRequest = _builder.build()

    /**
     * ```
     * Identity Provider to authenticate against via OIDC or SAML.
     * ```
     *
     * `.user_api.v1.Provider provider = 1;`
     */
    public var provider: user_api.v1.UserOuterClass.Provider
      @JvmName("getProvider")
      get() = _builder.getProvider()
      @JvmName("setProvider")
      set(value) {
        _builder.setProvider(value)
      }
    public var providerValue: kotlin.Int
      @JvmName("getProviderValue")
      get() = _builder.getProviderValue()
      @JvmName("setProviderValue")
      set(value) {
        _builder.setProviderValue(value)
      }
    /**
     * ```
     * Identity Provider to authenticate against via OIDC or SAML.
     * ```
     *
     * `.user_api.v1.Provider provider = 1;`
     */
    public fun clearProvider() {
      _builder.clearProvider()
    }

    /**
     * ```
     * Used by client to identify application instance initiating authentication.
     * ```
     *
     * `string instance_name = 2;`
     */
    public var instanceName: kotlin.String
      @JvmName("getInstanceName")
      get() = _builder.getInstanceName()
      @JvmName("setInstanceName")
      set(value) {
        _builder.setInstanceName(value)
      }
    /**
     * ```
     * Used by client to identify application instance initiating authentication.
     * ```
     *
     * `string instance_name = 2;`
     */
    public fun clearInstanceName() {
      _builder.clearInstanceName()
    }

    /**
     * ```
     * Used by client to identify origin of the request initiating authentication.
     * ```
     *
     * `string request_origin = 3;`
     */
    public var requestOrigin: kotlin.String
      @JvmName("getRequestOrigin")
      get() = _builder.getRequestOrigin()
      @JvmName("setRequestOrigin")
      set(value) {
        _builder.setRequestOrigin(value)
      }
    /**
     * ```
     * Used by client to identify origin of the request initiating authentication.
     * ```
     *
     * `string request_origin = 3;`
     */
    public fun clearRequestOrigin() {
      _builder.clearRequestOrigin()
    }

    /**
     * ```
     * Used by client to store state. 
     * ```
     *
     * `.google.protobuf.Struct client_state = 4;`
     */
    public var clientState: com.google.protobuf.Struct
      @JvmName("getClientState")
      get() = _builder.getClientState()
      @JvmName("setClientState")
      set(value) {
        _builder.setClientState(value)
      }
    /**
     * ```
     * Used by client to store state. 
     * ```
     *
     * `.google.protobuf.Struct client_state = 4;`
     */
    public fun clearClientState() {
      _builder.clearClientState()
    }
    /**
     * ```
     * Used by client to store state. 
     * ```
     *
     * `.google.protobuf.Struct client_state = 4;`
     * @return Whether the clientState field is set.
     */
    public fun hasClientState(): kotlin.Boolean {
      return _builder.hasClientState()
    }
    public val AuthnWithProviderRequestKt.Dsl.clientStateOrNull: com.google.protobuf.Struct?
      get() = _builder.clientStateOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.AuthnWithProviderRequest.copy(block: `user_api.v1`.AuthnWithProviderRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.AuthnWithProviderRequest =
  `user_api.v1`.AuthnWithProviderRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val user_api.v1.UserOuterClass.AuthnWithProviderRequestOrBuilder.clientStateOrNull: com.google.protobuf.Struct?
  get() = if (hasClientState()) getClientState() else null

