// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: vc_api/v1/vc.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package vc_api.v1;

@kotlin.jvm.JvmName("-initializepresentVerifiableCredentialRequest")
public inline fun presentVerifiableCredentialRequest(block: vc_api.v1.PresentVerifiableCredentialRequestKt.Dsl.() -> kotlin.Unit): vc_api.v1.Vc.PresentVerifiableCredentialRequest =
  vc_api.v1.PresentVerifiableCredentialRequestKt.Dsl._create(vc_api.v1.Vc.PresentVerifiableCredentialRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "presentation": "PRESENTATION", 
 * "nonce": "NonceLengthVariesByAlgo", 
 * "signature": "SIGNATURExBYTES=", 
 * "did": "did:method-name:zDIDMultibase58Encoded", 
 * "credential_type": [
 * 0, 
 * 1
 * ]
 * }
 * ```
 *
 * Protobuf type `vc_api.v1.PresentVerifiableCredentialRequest`
 */
public object PresentVerifiableCredentialRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: vc_api.v1.Vc.PresentVerifiableCredentialRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: vc_api.v1.Vc.PresentVerifiableCredentialRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): vc_api.v1.Vc.PresentVerifiableCredentialRequest = _builder.build()

    /**
     * ```
     * The presentation to be verified.
     * ```
     *
     * `string presentation = 1;`
     */
    public var presentation: kotlin.String
      @JvmName("getPresentation")
      get() = _builder.getPresentation()
      @JvmName("setPresentation")
      set(value) {
        _builder.setPresentation(value)
      }
    /**
     * ```
     * The presentation to be verified.
     * ```
     *
     * `string presentation = 1;`
     */
    public fun clearPresentation() {
      _builder.clearPresentation()
    }

    /**
     * ```
     * Value used for the challenge.
     * ```
     *
     * `string nonce = 2;`
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
     * `string nonce = 2;`
     */
    public fun clearNonce() {
      _builder.clearNonce()
    }

    /**
     * ```
     * Signed value of the challenge by the wallet.
     * ```
     *
     * `bytes signature = 3;`
     */
    public var signature: com.google.protobuf.ByteString
      @JvmName("getSignature")
      get() = _builder.getSignature()
      @JvmName("setSignature")
      set(value) {
        _builder.setSignature(value)
      }
    /**
     * ```
     * Signed value of the challenge by the wallet.
     * ```
     *
     * `bytes signature = 3;`
     */
    public fun clearSignature() {
      _builder.clearSignature()
    }

    /**
     * ```
     * W3C Decentralized Identifier (DID) of the wallet.
     * ```
     *
     * `string did = 4;`
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
     * `string did = 4;`
     */
    public fun clearDid() {
      _builder.clearDid()
    }

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class CredentialTypeProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_type = 5;`
     */
    public val credentialType: com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypeProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getCredentialTypeList()
      )
    /**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_type = 5;`
     * @param value The credentialType to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addCredentialType")
    public fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypeProxy>.add(value: vc_api.v1.Vc.CredentialType) {
      _builder.addCredentialType(value)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_type = 5;`
     * @param value The credentialType to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignCredentialType")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypeProxy>.plusAssign(value: vc_api.v1.Vc.CredentialType) {
      add(value)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_type = 5;`
     * @param values The credentialType to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllCredentialType")
    public fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypeProxy>.addAll(values: kotlin.collections.Iterable<vc_api.v1.Vc.CredentialType>) {
      _builder.addAllCredentialType(values)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_type = 5;`
     * @param values The credentialType to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllCredentialType")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypeProxy>.plusAssign(values: kotlin.collections.Iterable<vc_api.v1.Vc.CredentialType>) {
      addAll(values)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_type = 5;`
     * @param index The index to set the value at.
     * @param value The credentialType to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setCredentialType")
    public operator fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypeProxy>.set(index: kotlin.Int, value: vc_api.v1.Vc.CredentialType) {
      _builder.setCredentialType(index, value)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_type = 5;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearCredentialType")
    public fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypeProxy>.clear() {
      _builder.clearCredentialType()
    }}
}
@kotlin.jvm.JvmSynthetic
public inline fun vc_api.v1.Vc.PresentVerifiableCredentialRequest.copy(block: `vc_api.v1`.PresentVerifiableCredentialRequestKt.Dsl.() -> kotlin.Unit): vc_api.v1.Vc.PresentVerifiableCredentialRequest =
  `vc_api.v1`.PresentVerifiableCredentialRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()
