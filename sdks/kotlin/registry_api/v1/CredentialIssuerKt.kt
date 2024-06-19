// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: registry_api/v1/registry.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package registry_api.v1;

@kotlin.jvm.JvmName("-initializecredentialIssuer")
public inline fun credentialIssuer(block: registry_api.v1.CredentialIssuerKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CredentialIssuer =
  registry_api.v1.CredentialIssuerKt.Dsl._create(registry_api.v1.Registry.CredentialIssuer.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "issuer_did": "did:method-name:zDIDIssuerMultibase58Encoded",
 * "issuer_name": "ISSUER_NAME",
 * "email": "example@gmail.com",
 * "website": "google.com",
 * "postal_address": {
 * "country": "United States",
 * "city": "Anytown",
 * "state": "AL",
 * "postal_code": "10101",
 * "street": "123 Sesame Street"
 * },
 * "status": 0, 
 * "metadata": { 
 * "fields": [
 * {
 * "key": "VALUE"
 * }
 * ]
 * }
 * }
 * ```
 *
 * Protobuf type `registry_api.v1.CredentialIssuer`
 */
public object CredentialIssuerKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: registry_api.v1.Registry.CredentialIssuer.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: registry_api.v1.Registry.CredentialIssuer.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): registry_api.v1.Registry.CredentialIssuer = _builder.build()

    /**
     * ```
     * DID of Credential Issuer.
     * ```
     *
     * `string issuer_did = 1;`
     */
    public var issuerDid: kotlin.String
      @JvmName("getIssuerDid")
      get() = _builder.getIssuerDid()
      @JvmName("setIssuerDid")
      set(value) {
        _builder.setIssuerDid(value)
      }
    /**
     * ```
     * DID of Credential Issuer.
     * ```
     *
     * `string issuer_did = 1;`
     */
    public fun clearIssuerDid() {
      _builder.clearIssuerDid()
    }

    /**
     * ```
     * Name of Credential Issuer.
     * ```
     *
     * `string issuer_name = 2;`
     */
    public var issuerName: kotlin.String
      @JvmName("getIssuerName")
      get() = _builder.getIssuerName()
      @JvmName("setIssuerName")
      set(value) {
        _builder.setIssuerName(value)
      }
    /**
     * ```
     * Name of Credential Issuer.
     * ```
     *
     * `string issuer_name = 2;`
     */
    public fun clearIssuerName() {
      _builder.clearIssuerName()
    }

    /**
     * ```
     * Email of Credential Issuer.
     * ```
     *
     * `string email = 3;`
     */
    public var email: kotlin.String
      @JvmName("getEmail")
      get() = _builder.getEmail()
      @JvmName("setEmail")
      set(value) {
        _builder.setEmail(value)
      }
    /**
     * ```
     * Email of Credential Issuer.
     * ```
     *
     * `string email = 3;`
     */
    public fun clearEmail() {
      _builder.clearEmail()
    }

    /**
     * ```
     * Website of Credential Issuer.
     * ```
     *
     * `string website = 4;`
     */
    public var website: kotlin.String
      @JvmName("getWebsite")
      get() = _builder.getWebsite()
      @JvmName("setWebsite")
      set(value) {
        _builder.setWebsite(value)
      }
    /**
     * ```
     * Website of Credential Issuer.
     * ```
     *
     * `string website = 4;`
     */
    public fun clearWebsite() {
      _builder.clearWebsite()
    }

    /**
     * ```
     * Postal Address of Credential Issuer.
     * ```
     *
     * `.registry_api.v1.PostalAddress postal_address = 5;`
     */
    public var postalAddress: registry_api.v1.Registry.PostalAddress
      @JvmName("getPostalAddress")
      get() = _builder.getPostalAddress()
      @JvmName("setPostalAddress")
      set(value) {
        _builder.setPostalAddress(value)
      }
    /**
     * ```
     * Postal Address of Credential Issuer.
     * ```
     *
     * `.registry_api.v1.PostalAddress postal_address = 5;`
     */
    public fun clearPostalAddress() {
      _builder.clearPostalAddress()
    }
    /**
     * ```
     * Postal Address of Credential Issuer.
     * ```
     *
     * `.registry_api.v1.PostalAddress postal_address = 5;`
     * @return Whether the postalAddress field is set.
     */
    public fun hasPostalAddress(): kotlin.Boolean {
      return _builder.hasPostalAddress()
    }
    public val CredentialIssuerKt.Dsl.postalAddressOrNull: registry_api.v1.Registry.PostalAddress?
      get() = _builder.postalAddressOrNull

    /**
     * ```
     * Status of Credential Issuer, e.g. Active, Suspended.
     * ```
     *
     * `.registry_api.v1.IssuerStatus status = 6;`
     */
    public var status: registry_api.v1.Registry.IssuerStatus
      @JvmName("getStatus")
      get() = _builder.getStatus()
      @JvmName("setStatus")
      set(value) {
        _builder.setStatus(value)
      }
    public var statusValue: kotlin.Int
      @JvmName("getStatusValue")
      get() = _builder.getStatusValue()
      @JvmName("setStatusValue")
      set(value) {
        _builder.setStatusValue(value)
      }
    /**
     * ```
     * Status of Credential Issuer, e.g. Active, Suspended.
     * ```
     *
     * `.registry_api.v1.IssuerStatus status = 6;`
     */
    public fun clearStatus() {
      _builder.clearStatus()
    }

    /**
     * ```
     * Metadata of Credential Issuer.
     * ```
     *
     * `.google.protobuf.Struct metadata = 7;`
     */
    public var metadata: com.google.protobuf.Struct
      @JvmName("getMetadata")
      get() = _builder.getMetadata()
      @JvmName("setMetadata")
      set(value) {
        _builder.setMetadata(value)
      }
    /**
     * ```
     * Metadata of Credential Issuer.
     * ```
     *
     * `.google.protobuf.Struct metadata = 7;`
     */
    public fun clearMetadata() {
      _builder.clearMetadata()
    }
    /**
     * ```
     * Metadata of Credential Issuer.
     * ```
     *
     * `.google.protobuf.Struct metadata = 7;`
     * @return Whether the metadata field is set.
     */
    public fun hasMetadata(): kotlin.Boolean {
      return _builder.hasMetadata()
    }
    public val CredentialIssuerKt.Dsl.metadataOrNull: com.google.protobuf.Struct?
      get() = _builder.metadataOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun registry_api.v1.Registry.CredentialIssuer.copy(block: `registry_api.v1`.CredentialIssuerKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CredentialIssuer =
  `registry_api.v1`.CredentialIssuerKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val registry_api.v1.Registry.CredentialIssuerOrBuilder.postalAddressOrNull: registry_api.v1.Registry.PostalAddress?
  get() = if (hasPostalAddress()) getPostalAddress() else null

public val registry_api.v1.Registry.CredentialIssuerOrBuilder.metadataOrNull: com.google.protobuf.Struct?
  get() = if (hasMetadata()) getMetadata() else null
