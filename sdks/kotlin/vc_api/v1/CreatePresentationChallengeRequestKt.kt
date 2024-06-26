// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: vc_api/v1/vc.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package vc_api.v1;

@kotlin.jvm.JvmName("-initializecreatePresentationChallengeRequest")
public inline fun createPresentationChallengeRequest(block: vc_api.v1.CreatePresentationChallengeRequestKt.Dsl.() -> kotlin.Unit): vc_api.v1.Vc.CreatePresentationChallengeRequest =
  vc_api.v1.CreatePresentationChallengeRequestKt.Dsl._create(vc_api.v1.Vc.CreatePresentationChallengeRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "credential_types": [
 * 0,
 * 1
 * ]
 * }
 * ```
 *
 * Protobuf type `vc_api.v1.CreatePresentationChallengeRequest`
 */
public object CreatePresentationChallengeRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: vc_api.v1.Vc.CreatePresentationChallengeRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: vc_api.v1.Vc.CreatePresentationChallengeRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): vc_api.v1.Vc.CreatePresentationChallengeRequest = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class CredentialTypesProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_types = 1;`
     */
    public val credentialTypes: com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypesProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getCredentialTypesList()
      )
    /**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_types = 1;`
     * @param value The credentialTypes to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addCredentialTypes")
    public fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypesProxy>.add(value: vc_api.v1.Vc.CredentialType) {
      _builder.addCredentialTypes(value)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_types = 1;`
     * @param value The credentialTypes to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignCredentialTypes")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypesProxy>.plusAssign(value: vc_api.v1.Vc.CredentialType) {
      add(value)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_types = 1;`
     * @param values The credentialTypes to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllCredentialTypes")
    public fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypesProxy>.addAll(values: kotlin.collections.Iterable<vc_api.v1.Vc.CredentialType>) {
      _builder.addAllCredentialTypes(values)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_types = 1;`
     * @param values The credentialTypes to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllCredentialTypes")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypesProxy>.plusAssign(values: kotlin.collections.Iterable<vc_api.v1.Vc.CredentialType>) {
      addAll(values)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_types = 1;`
     * @param index The index to set the value at.
     * @param value The credentialTypes to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setCredentialTypes")
    public operator fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypesProxy>.set(index: kotlin.Int, value: vc_api.v1.Vc.CredentialType) {
      _builder.setCredentialTypes(index, value)
    }/**
     * ```
     * Type(s) of the verifiable credential(s) being presented.
     * ```
     *
     * `repeated .vc_api.v1.CredentialType credential_types = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearCredentialTypes")
    public fun com.google.protobuf.kotlin.DslList<vc_api.v1.Vc.CredentialType, CredentialTypesProxy>.clear() {
      _builder.clearCredentialTypes()
    }}
}
@kotlin.jvm.JvmSynthetic
public inline fun vc_api.v1.Vc.CreatePresentationChallengeRequest.copy(block: `vc_api.v1`.CreatePresentationChallengeRequestKt.Dsl.() -> kotlin.Unit): vc_api.v1.Vc.CreatePresentationChallengeRequest =
  `vc_api.v1`.CreatePresentationChallengeRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

