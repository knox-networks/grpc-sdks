// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: registry_api/v1/registry.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package registry_api.v1;

@kotlin.jvm.JvmName("-initializestatusListEntry")
public inline fun statusListEntry(block: registry_api.v1.StatusListEntryKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.StatusListEntry =
  registry_api.v1.StatusListEntryKt.Dsl._create(registry_api.v1.Registry.StatusListEntry.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * { 
 * "credential_id": "https://issuer.oidp.uscis.gov/credentials/83627465",
 * "credential_issuer_id": "did:method-name:zDIDCredentialIssuerMultibase58Encoded",
 * "status_purpose": 0,
 * "credential_type": "BankAccount",
 * "status_list_index": 0,
 * "status_list_credential_id": "https://reg.YOUR_SANDBOX_URL.knoxnetworks.io/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
 * "status": true
 * }
 * ```
 *
 * Protobuf type `registry_api.v1.StatusListEntry`
 */
public object StatusListEntryKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: registry_api.v1.Registry.StatusListEntry.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: registry_api.v1.Registry.StatusListEntry.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): registry_api.v1.Registry.StatusListEntry = _builder.build()

    /**
     * ```
     * Credential Unique ID, e.g. "https://issuer.oidp.uscis.gov/credentials/83627465"
     * ```
     *
     * `string credential_id = 1;`
     */
    public var credentialId: kotlin.String
      @JvmName("getCredentialId")
      get() = _builder.getCredentialId()
      @JvmName("setCredentialId")
      set(value) {
        _builder.setCredentialId(value)
      }
    /**
     * ```
     * Credential Unique ID, e.g. "https://issuer.oidp.uscis.gov/credentials/83627465"
     * ```
     *
     * `string credential_id = 1;`
     */
    public fun clearCredentialId() {
      _builder.clearCredentialId()
    }

    /**
     * ```
     * Credential Issuer DID.
     * ```
     *
     * `string credential_issuer_id = 2;`
     */
    public var credentialIssuerId: kotlin.String
      @JvmName("getCredentialIssuerId")
      get() = _builder.getCredentialIssuerId()
      @JvmName("setCredentialIssuerId")
      set(value) {
        _builder.setCredentialIssuerId(value)
      }
    /**
     * ```
     * Credential Issuer DID.
     * ```
     *
     * `string credential_issuer_id = 2;`
     */
    public fun clearCredentialIssuerId() {
      _builder.clearCredentialIssuerId()
    }

    /**
     * ```
     * Status Purpose, e.g. revocation.
     * ```
     *
     * `.registry_api.v1.StatusPurpose status_purpose = 3;`
     */
    public var statusPurpose: registry_api.v1.Registry.StatusPurpose
      @JvmName("getStatusPurpose")
      get() = _builder.getStatusPurpose()
      @JvmName("setStatusPurpose")
      set(value) {
        _builder.setStatusPurpose(value)
      }
    public var statusPurposeValue: kotlin.Int
      @JvmName("getStatusPurposeValue")
      get() = _builder.getStatusPurposeValue()
      @JvmName("setStatusPurposeValue")
      set(value) {
        _builder.setStatusPurposeValue(value)
      }
    /**
     * ```
     * Status Purpose, e.g. revocation.
     * ```
     *
     * `.registry_api.v1.StatusPurpose status_purpose = 3;`
     */
    public fun clearStatusPurpose() {
      _builder.clearStatusPurpose()
    }

    /**
     * ```
     * Credential Type.
     * ```
     *
     * `string credential_type = 4;`
     */
    public var credentialType: kotlin.String
      @JvmName("getCredentialType")
      get() = _builder.getCredentialType()
      @JvmName("setCredentialType")
      set(value) {
        _builder.setCredentialType(value)
      }
    /**
     * ```
     * Credential Type.
     * ```
     *
     * `string credential_type = 4;`
     */
    public fun clearCredentialType() {
      _builder.clearCredentialType()
    }

    /**
     * ```
     * Status Index of the VC.
     * ```
     *
     * `int64 status_list_index = 5;`
     */
    public var statusListIndex: kotlin.Long
      @JvmName("getStatusListIndex")
      get() = _builder.getStatusListIndex()
      @JvmName("setStatusListIndex")
      set(value) {
        _builder.setStatusListIndex(value)
      }
    /**
     * ```
     * Status Index of the VC.
     * ```
     *
     * `int64 status_list_index = 5;`
     */
    public fun clearStatusListIndex() {
      _builder.clearStatusListIndex()
    }

    /**
     * ```
     * Credential Status List ID, e.g. "https://reg.YOUR_SANDBOX_URL.knoxnetworks.io/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
     * ```
     *
     * `string status_list_credential_id = 6;`
     */
    public var statusListCredentialId: kotlin.String
      @JvmName("getStatusListCredentialId")
      get() = _builder.getStatusListCredentialId()
      @JvmName("setStatusListCredentialId")
      set(value) {
        _builder.setStatusListCredentialId(value)
      }
    /**
     * ```
     * Credential Status List ID, e.g. "https://reg.YOUR_SANDBOX_URL.knoxnetworks.io/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
     * ```
     *
     * `string status_list_credential_id = 6;`
     */
    public fun clearStatusListCredentialId() {
      _builder.clearStatusListCredentialId()
    }

    /**
     * ```
     * Status.
     * ```
     *
     * `bool status = 7;`
     */
    public var status: kotlin.Boolean
      @JvmName("getStatus")
      get() = _builder.getStatus()
      @JvmName("setStatus")
      set(value) {
        _builder.setStatus(value)
      }
    /**
     * ```
     * Status.
     * ```
     *
     * `bool status = 7;`
     */
    public fun clearStatus() {
      _builder.clearStatus()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun registry_api.v1.Registry.StatusListEntry.copy(block: `registry_api.v1`.StatusListEntryKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.StatusListEntry =
  `registry_api.v1`.StatusListEntryKt.Dsl._create(this.toBuilder()).apply { block() }._build()

