// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: issuer_api/v1/issuer.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package issuer_api.v1;

@kotlin.jvm.JvmName("-initializeauthenticateRequest")
public inline fun authenticateRequest(block: issuer_api.v1.AuthenticateRequestKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.AuthenticateRequest =
  issuer_api.v1.AuthenticateRequestKt.Dsl._create(issuer_api.v1.Issuer.AuthenticateRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "verifier": {
 * "signature_system": 1,
 * "verifier": "VerifierLengthChangesByAlgo="
 * }
 * }
 * ```
 *
 * Protobuf type `issuer_api.v1.AuthenticateRequest`
 */
public object AuthenticateRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: issuer_api.v1.Issuer.AuthenticateRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: issuer_api.v1.Issuer.AuthenticateRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): issuer_api.v1.Issuer.AuthenticateRequest = _builder.build()

    /**
     * ```
     * Verifier of the requester.
     * ```
     *
     * `.common.DynamicVerifier verifier = 1;`
     */
    public var verifier: common.Common.DynamicVerifier
      @JvmName("getVerifier")
      get() = _builder.getVerifier()
      @JvmName("setVerifier")
      set(value) {
        _builder.setVerifier(value)
      }
    /**
     * ```
     * Verifier of the requester.
     * ```
     *
     * `.common.DynamicVerifier verifier = 1;`
     */
    public fun clearVerifier() {
      _builder.clearVerifier()
    }
    /**
     * ```
     * Verifier of the requester.
     * ```
     *
     * `.common.DynamicVerifier verifier = 1;`
     * @return Whether the verifier field is set.
     */
    public fun hasVerifier(): kotlin.Boolean {
      return _builder.hasVerifier()
    }
    public val AuthenticateRequestKt.Dsl.verifierOrNull: common.Common.DynamicVerifier?
      get() = _builder.verifierOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun issuer_api.v1.Issuer.AuthenticateRequest.copy(block: `issuer_api.v1`.AuthenticateRequestKt.Dsl.() -> kotlin.Unit): issuer_api.v1.Issuer.AuthenticateRequest =
  `issuer_api.v1`.AuthenticateRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val issuer_api.v1.Issuer.AuthenticateRequestOrBuilder.verifierOrNull: common.Common.DynamicVerifier?
  get() = if (hasVerifier()) getVerifier() else null

