// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializehandleSAMLCallbackResponse")
public inline fun handleSAMLCallbackResponse(block: user_api.v1.HandleSAMLCallbackResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.HandleSAMLCallbackResponse =
  user_api.v1.HandleSAMLCallbackResponseKt.Dsl._create(user_api.v1.UserOuterClass.HandleSAMLCallbackResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "user": {
 * "uuid": "UUID", 
 * "first_name": "John", 
 * "last_name": "Doe", 
 * "email": "example@gmail.com", 
 * "role": 0, 
 * "avatar": "AVATAR_URL", 
 * "phone": "+1 555 555 5555", 
 * "dids": [
 * "did:method-name:zDIDMultibase58Encoded", 
 * "did:method-name:zDIDMultibase58Encoded"
 * ], 
 * "external_ids": [
 * {
 * "key": "KEY1", 
 * "value": "VALUE1"
 * }, 
 * {
 * "key": "KEY2", 
 * "value": "VALUE2"
 * }
 * ], 
 * "metadata": { 
 * "fields": []
 * },
 * "account": "15436675"
 * },
 * "auth_token": { 
 * "token": "TOKEN", 
 * "token_type": "TOKEN_TYPE", 
 * "expires_in": 100, 
 * "refresh_token": "REFRESH_TOKEN"
 * }, 
 * "instance_name": "INSTANCE_NAME", 
 * "request_origin": "REQUEST_ORIGIN"
 * }
 * ```
 *
 * Protobuf type `user_api.v1.HandleSAMLCallbackResponse`
 */
public object HandleSAMLCallbackResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.HandleSAMLCallbackResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.HandleSAMLCallbackResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.HandleSAMLCallbackResponse = _builder.build()

    /**
     * ```
     * User object of the subject.
     * ```
     *
     * `.user_api.v1.User user = 1;`
     */
    public var user: user_api.v1.UserOuterClass.User
      @JvmName("getUser")
      get() = _builder.getUser()
      @JvmName("setUser")
      set(value) {
        _builder.setUser(value)
      }
    /**
     * ```
     * User object of the subject.
     * ```
     *
     * `.user_api.v1.User user = 1;`
     */
    public fun clearUser() {
      _builder.clearUser()
    }
    /**
     * ```
     * User object of the subject.
     * ```
     *
     * `.user_api.v1.User user = 1;`
     * @return Whether the user field is set.
     */
    public fun hasUser(): kotlin.Boolean {
      return _builder.hasUser()
    }
    public val HandleSAMLCallbackResponseKt.Dsl.userOrNull: user_api.v1.UserOuterClass.User?
      get() = _builder.userOrNull

    /**
     * ```
     * OAuth2 RFC6749 Access Token
     * ```
     *
     * `.user_api.v1.AuthToken auth_token = 2;`
     */
    public var authToken: user_api.v1.UserOuterClass.AuthToken
      @JvmName("getAuthToken")
      get() = _builder.getAuthToken()
      @JvmName("setAuthToken")
      set(value) {
        _builder.setAuthToken(value)
      }
    /**
     * ```
     * OAuth2 RFC6749 Access Token
     * ```
     *
     * `.user_api.v1.AuthToken auth_token = 2;`
     */
    public fun clearAuthToken() {
      _builder.clearAuthToken()
    }
    /**
     * ```
     * OAuth2 RFC6749 Access Token
     * ```
     *
     * `.user_api.v1.AuthToken auth_token = 2;`
     * @return Whether the authToken field is set.
     */
    public fun hasAuthToken(): kotlin.Boolean {
      return _builder.hasAuthToken()
    }
    public val HandleSAMLCallbackResponseKt.Dsl.authTokenOrNull: user_api.v1.UserOuterClass.AuthToken?
      get() = _builder.authTokenOrNull

    /**
     * ```
     * Used by client to identify application instance initiating authentication.
     * ```
     *
     * `string instance_name = 3;`
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
     * `string instance_name = 3;`
     */
    public fun clearInstanceName() {
      _builder.clearInstanceName()
    }

    /**
     * ```
     * Used by client to identify origin of the request initiating authentication.
     * ```
     *
     * `string request_origin = 4;`
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
     * `string request_origin = 4;`
     */
    public fun clearRequestOrigin() {
      _builder.clearRequestOrigin()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.HandleSAMLCallbackResponse.copy(block: `user_api.v1`.HandleSAMLCallbackResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.HandleSAMLCallbackResponse =
  `user_api.v1`.HandleSAMLCallbackResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val user_api.v1.UserOuterClass.HandleSAMLCallbackResponseOrBuilder.userOrNull: user_api.v1.UserOuterClass.User?
  get() = if (hasUser()) getUser() else null

public val user_api.v1.UserOuterClass.HandleSAMLCallbackResponseOrBuilder.authTokenOrNull: user_api.v1.UserOuterClass.AuthToken?
  get() = if (hasAuthToken()) getAuthToken() else null
