// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializefindByEmailResponse")
public inline fun findByEmailResponse(block: user_api.v1.FindByEmailResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.FindByEmailResponse =
  user_api.v1.FindByEmailResponseKt.Dsl._create(user_api.v1.UserOuterClass.FindByEmailResponse.newBuilder()).apply { block() }._build()
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
 * }
 * }
 * ```
 *
 * Protobuf type `user_api.v1.FindByEmailResponse`
 */
public object FindByEmailResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.FindByEmailResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.FindByEmailResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.FindByEmailResponse = _builder.build()

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
    public val FindByEmailResponseKt.Dsl.userOrNull: user_api.v1.UserOuterClass.User?
      get() = _builder.userOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.FindByEmailResponse.copy(block: `user_api.v1`.FindByEmailResponseKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.FindByEmailResponse =
  `user_api.v1`.FindByEmailResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val user_api.v1.UserOuterClass.FindByEmailResponseOrBuilder.userOrNull: user_api.v1.UserOuterClass.User?
  get() = if (hasUser()) getUser() else null

