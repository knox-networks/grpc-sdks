// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: registry_api/v1/registry.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package registry_api.v1;

@kotlin.jvm.JvmName("-initializecreateStatusListEntryResponse")
public inline fun createStatusListEntryResponse(block: registry_api.v1.CreateStatusListEntryResponseKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CreateStatusListEntryResponse =
  registry_api.v1.CreateStatusListEntryResponseKt.Dsl._create(registry_api.v1.Registry.CreateStatusListEntryResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * { 
 * "status_list_entry": { 
 * "credential_id": "https://issuer.oidp.uscis.gov/credentials/83627465",
 * "credential_issuer_id": "did:method-name:zDIDCredentialIssuerMultibase58Encoded",
 * "status_purpose": 0,
 * "credential_type": "BankAccount",
 * "status_list_index": 0,
 * "status_list_credential_id": "https://reg.YOUR_SANDBOX_URL.knoxnetworks.io/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
 * "status": true
 * }
 * }
 * ```
 *
 * Protobuf type `registry_api.v1.CreateStatusListEntryResponse`
 */
public object CreateStatusListEntryResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: registry_api.v1.Registry.CreateStatusListEntryResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: registry_api.v1.Registry.CreateStatusListEntryResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): registry_api.v1.Registry.CreateStatusListEntryResponse = _builder.build()

    /**
     * ```
     * Created StatusListEntry.
     * ```
     *
     * `.registry_api.v1.StatusListEntry status_list_entry = 1;`
     */
    public var statusListEntry: registry_api.v1.Registry.StatusListEntry
      @JvmName("getStatusListEntry")
      get() = _builder.getStatusListEntry()
      @JvmName("setStatusListEntry")
      set(value) {
        _builder.setStatusListEntry(value)
      }
    /**
     * ```
     * Created StatusListEntry.
     * ```
     *
     * `.registry_api.v1.StatusListEntry status_list_entry = 1;`
     */
    public fun clearStatusListEntry() {
      _builder.clearStatusListEntry()
    }
    /**
     * ```
     * Created StatusListEntry.
     * ```
     *
     * `.registry_api.v1.StatusListEntry status_list_entry = 1;`
     * @return Whether the statusListEntry field is set.
     */
    public fun hasStatusListEntry(): kotlin.Boolean {
      return _builder.hasStatusListEntry()
    }
    public val CreateStatusListEntryResponseKt.Dsl.statusListEntryOrNull: registry_api.v1.Registry.StatusListEntry?
      get() = _builder.statusListEntryOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun registry_api.v1.Registry.CreateStatusListEntryResponse.copy(block: `registry_api.v1`.CreateStatusListEntryResponseKt.Dsl.() -> kotlin.Unit): registry_api.v1.Registry.CreateStatusListEntryResponse =
  `registry_api.v1`.CreateStatusListEntryResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val registry_api.v1.Registry.CreateStatusListEntryResponseOrBuilder.statusListEntryOrNull: registry_api.v1.Registry.StatusListEntry?
  get() = if (hasStatusListEntry()) getStatusListEntry() else null
