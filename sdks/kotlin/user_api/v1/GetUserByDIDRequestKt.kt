// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializegetUserByDIDRequest")
public inline fun getUserByDIDRequest(block: user_api.v1.GetUserByDIDRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.GetUserByDIDRequest =
  user_api.v1.GetUserByDIDRequestKt.Dsl._create(user_api.v1.UserOuterClass.GetUserByDIDRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "did": "did:method-name:zDIDMultibase58Encoded"
 * } 
 * ```
 *
 * Protobuf type `user_api.v1.GetUserByDIDRequest`
 */
public object GetUserByDIDRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.GetUserByDIDRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.GetUserByDIDRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.GetUserByDIDRequest = _builder.build()

    /**
     * ```
     * W3C Decentralized Identifier (DID) of the wallet.
     * ```
     *
     * `string did = 1;`
     */
    public var did: kotlin.String
      @JvmName("getDid")
      get() = _builder.getDid()
      @JvmName("setDid")
      set(value) {
        _builder.setDid(value)
      }
    /**
     * ```
     * W3C Decentralized Identifier (DID) of the wallet.
     * ```
     *
     * `string did = 1;`
     */
    public fun clearDid() {
      _builder.clearDid()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.GetUserByDIDRequest.copy(block: `user_api.v1`.GetUserByDIDRequestKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.GetUserByDIDRequest =
  `user_api.v1`.GetUserByDIDRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

