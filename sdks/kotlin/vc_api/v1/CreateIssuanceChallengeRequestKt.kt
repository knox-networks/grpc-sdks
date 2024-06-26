// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: vc_api/v1/vc.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package vc_api.v1;

@kotlin.jvm.JvmName("-initializecreateIssuanceChallengeRequest")
public inline fun createIssuanceChallengeRequest(block: vc_api.v1.CreateIssuanceChallengeRequestKt.Dsl.() -> kotlin.Unit): vc_api.v1.Vc.CreateIssuanceChallengeRequest =
  vc_api.v1.CreateIssuanceChallengeRequestKt.Dsl._create(vc_api.v1.Vc.CreateIssuanceChallengeRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "credential_type": 1, 
 * "did": "did:method-name:zDIDMultibase58Encoded"
 * }
 * ```
 *
 * Protobuf type `vc_api.v1.CreateIssuanceChallengeRequest`
 */
public object CreateIssuanceChallengeRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: vc_api.v1.Vc.CreateIssuanceChallengeRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: vc_api.v1.Vc.CreateIssuanceChallengeRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): vc_api.v1.Vc.CreateIssuanceChallengeRequest = _builder.build()

    /**
     * ```
     * Type of the verifiable credential being requested for issuance.
     * ```
     *
     * `.vc_api.v1.CredentialType credential_type = 1;`
     */
    public var credentialType: vc_api.v1.Vc.CredentialType
      @JvmName("getCredentialType")
      get() = _builder.getCredentialType()
      @JvmName("setCredentialType")
      set(value) {
        _builder.setCredentialType(value)
      }
    public var credentialTypeValue: kotlin.Int
      @JvmName("getCredentialTypeValue")
      get() = _builder.getCredentialTypeValue()
      @JvmName("setCredentialTypeValue")
      set(value) {
        _builder.setCredentialTypeValue(value)
      }
    /**
     * ```
     * Type of the verifiable credential being requested for issuance.
     * ```
     *
     * `.vc_api.v1.CredentialType credential_type = 1;`
     */
    public fun clearCredentialType() {
      _builder.clearCredentialType()
    }

    /**
     * ```
     * W3C Decentralized Identifier (DID) of the wallet.
     * ```
     *
     * `string did = 2;`
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
     * `string did = 2;`
     */
    public fun clearDid() {
      _builder.clearDid()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun vc_api.v1.Vc.CreateIssuanceChallengeRequest.copy(block: `vc_api.v1`.CreateIssuanceChallengeRequestKt.Dsl.() -> kotlin.Unit): vc_api.v1.Vc.CreateIssuanceChallengeRequest =
  `vc_api.v1`.CreateIssuanceChallengeRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

