// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializeconfirmUserRequest")
public inline fun confirmUserRequest(block: user_api.v1.ConfirmUserRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.ConfirmUserRequest =
  user_api.v1.ConfirmUserRequestKt.Dsl._create(user_api.v1.UserOuterClass.ConfirmUserRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "email_or_phone": "EMAIL_OR_PHONE", 
 * "token": "TOKEN"
 * }
 * ```
 *
 * Protobuf type `user_api.v1.ConfirmUserRequest`
 */
public object ConfirmUserRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.ConfirmUserRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.ConfirmUserRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.ConfirmUserRequest = _builder.build()

    /**
     * ```
     * Confirmation method for user registration.
     * ```
     *
     * `string email_or_phone = 1;`
     */
    public var emailOrPhone: kotlin.String
      @JvmName("getEmailOrPhone")
      get() = _builder.getEmailOrPhone()
      @JvmName("setEmailOrPhone")
      set(value) {
        _builder.setEmailOrPhone(value)
      }
    /**
     * ```
     * Confirmation method for user registration.
     * ```
     *
     * `string email_or_phone = 1;`
     */
    public fun clearEmailOrPhone() {
      _builder.clearEmailOrPhone()
    }

    /**
     * ```
     * Token received from previous request.
     * ```
     *
     * `string token = 2;`
     */
    public var token: kotlin.String
      @JvmName("getToken")
      get() = _builder.getToken()
      @JvmName("setToken")
      set(value) {
        _builder.setToken(value)
      }
    /**
     * ```
     * Token received from previous request.
     * ```
     *
     * `string token = 2;`
     */
    public fun clearToken() {
      _builder.clearToken()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.ConfirmUserRequest.copy(block: `user_api.v1`.ConfirmUserRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.ConfirmUserRequest =
  `user_api.v1`.ConfirmUserRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

