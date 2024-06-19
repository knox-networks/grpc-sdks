// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializecreateRegisterWalletChallengeResponse")
public inline fun createRegisterWalletChallengeResponse(block: user_api.v1.CreateRegisterWalletChallengeResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.CreateRegisterWalletChallengeResponse =
  user_api.v1.CreateRegisterWalletChallengeResponseKt.Dsl._create(user_api.v1.UserOuterClass.CreateRegisterWalletChallengeResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "registration_start": { 
 * "nonce": "NONCE" 
 * }
 * } 
 * ```
 *
 * Protobuf type `user_api.v1.CreateRegisterWalletChallengeResponse`
 */
public object CreateRegisterWalletChallengeResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.CreateRegisterWalletChallengeResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.CreateRegisterWalletChallengeResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.CreateRegisterWalletChallengeResponse = _builder.build()

    /**
     * ```
     * Value used for the challenge.
     * ```
     *
     * `string nonce = 1;`
     */
    public var nonce: kotlin.String
      @JvmName("getNonce")
      get() = _builder.getNonce()
      @JvmName("setNonce")
      set(value) {
        _builder.setNonce(value)
      }
    /**
     * ```
     * Value used for the challenge.
     * ```
     *
     * `string nonce = 1;`
     */
    public fun clearNonce() {
      _builder.clearNonce()
    }
    /**
     * ```
     * Value used for the challenge.
     * ```
     *
     * `string nonce = 1;`
     * @return Whether the nonce field is set.
     */
    public fun hasNonce(): kotlin.Boolean {
      return _builder.hasNonce()
    }

    /**
     * ```
     * Confirmation of successful challenge.
     * ```
     *
     * `string ok = 2;`
     */
    public var ok: kotlin.String
      @JvmName("getOk")
      get() = _builder.getOk()
      @JvmName("setOk")
      set(value) {
        _builder.setOk(value)
      }
    /**
     * ```
     * Confirmation of successful challenge.
     * ```
     *
     * `string ok = 2;`
     */
    public fun clearOk() {
      _builder.clearOk()
    }
    /**
     * ```
     * Confirmation of successful challenge.
     * ```
     *
     * `string ok = 2;`
     * @return Whether the ok field is set.
     */
    public fun hasOk(): kotlin.Boolean {
      return _builder.hasOk()
    }
    public val registrationStartCase: user_api.v1.UserOuterClass.CreateRegisterWalletChallengeResponse.RegistrationStartCase
      @JvmName("getRegistrationStartCase")
      get() = _builder.getRegistrationStartCase()

    public fun clearRegistrationStart() {
      _builder.clearRegistrationStart()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.CreateRegisterWalletChallengeResponse.copy(block: `user_api.v1`.CreateRegisterWalletChallengeResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.CreateRegisterWalletChallengeResponse =
  `user_api.v1`.CreateRegisterWalletChallengeResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()
