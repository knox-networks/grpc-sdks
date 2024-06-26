// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializeuser")
public inline fun user(block: user_api.v1.UserKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.User =
  user_api.v1.UserKt.Dsl._create(user_api.v1.UserOuterClass.User.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
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
 * "fields": [
 * {
 * "key": "VALUE"
 * }
 * ]
 * },
 * "account": "15436675"
 * }
 * ```
 *
 * Protobuf type `user_api.v1.User`
 */
public object UserKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.User.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.User.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.User = _builder.build()

    /**
     * ```
     * Unique identifier of the user for internal use.
     * ```
     *
     * `string uuid = 1;`
     */
    public var uuid: kotlin.String
      @JvmName("getUuid")
      get() = _builder.getUuid()
      @JvmName("setUuid")
      set(value) {
        _builder.setUuid(value)
      }
    /**
     * ```
     * Unique identifier of the user for internal use.
     * ```
     *
     * `string uuid = 1;`
     */
    public fun clearUuid() {
      _builder.clearUuid()
    }

    /**
     * ```
     * First name of the user.
     * ```
     *
     * `string first_name = 2;`
     */
    public var firstName: kotlin.String
      @JvmName("getFirstName")
      get() = _builder.getFirstName()
      @JvmName("setFirstName")
      set(value) {
        _builder.setFirstName(value)
      }
    /**
     * ```
     * First name of the user.
     * ```
     *
     * `string first_name = 2;`
     */
    public fun clearFirstName() {
      _builder.clearFirstName()
    }

    /**
     * ```
     * Last name of the user.
     * ```
     *
     * `string last_name = 3;`
     */
    public var lastName: kotlin.String
      @JvmName("getLastName")
      get() = _builder.getLastName()
      @JvmName("setLastName")
      set(value) {
        _builder.setLastName(value)
      }
    /**
     * ```
     * Last name of the user.
     * ```
     *
     * `string last_name = 3;`
     */
    public fun clearLastName() {
      _builder.clearLastName()
    }

    /**
     * ```
     * Email address of the user.
     * ```
     *
     * `string email = 6;`
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
     * Email address of the user.
     * ```
     *
     * `string email = 6;`
     */
    public fun clearEmail() {
      _builder.clearEmail()
    }

    /**
     * ```
     * Role of the user, either but not limited to Admin or User.
     * ```
     *
     * `.user_api.v1.Role role = 7;`
     */
    public var role: user_api.v1.UserOuterClass.Role
      @JvmName("getRole")
      get() = _builder.getRole()
      @JvmName("setRole")
      set(value) {
        _builder.setRole(value)
      }
    public var roleValue: kotlin.Int
      @JvmName("getRoleValue")
      get() = _builder.getRoleValue()
      @JvmName("setRoleValue")
      set(value) {
        _builder.setRoleValue(value)
      }
    /**
     * ```
     * Role of the user, either but not limited to Admin or User.
     * ```
     *
     * `.user_api.v1.Role role = 7;`
     */
    public fun clearRole() {
      _builder.clearRole()
    }

    /**
     * ```
     * Avatar URL of the user.
     * ```
     *
     * `string avatar = 8;`
     */
    public var avatar: kotlin.String
      @JvmName("getAvatar")
      get() = _builder.getAvatar()
      @JvmName("setAvatar")
      set(value) {
        _builder.setAvatar(value)
      }
    /**
     * ```
     * Avatar URL of the user.
     * ```
     *
     * `string avatar = 8;`
     */
    public fun clearAvatar() {
      _builder.clearAvatar()
    }

    /**
     * ```
     * Phone number of the user.
     * ```
     *
     * `string phone = 9;`
     */
    public var phone: kotlin.String
      @JvmName("getPhone")
      get() = _builder.getPhone()
      @JvmName("setPhone")
      set(value) {
        _builder.setPhone(value)
      }
    /**
     * ```
     * Phone number of the user.
     * ```
     *
     * `string phone = 9;`
     */
    public fun clearPhone() {
      _builder.clearPhone()
    }

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class DidsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * W3C Decentralized Identifiers (DIDs) associated to the user.
     * ```
     *
     * `repeated string dids = 10;`
     * @return A list containing the dids.
     */
    public val dids: com.google.protobuf.kotlin.DslList<kotlin.String, DidsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getDidsList()
      )
    /**
     * ```
     * W3C Decentralized Identifiers (DIDs) associated to the user.
     * ```
     *
     * `repeated string dids = 10;`
     * @param value The dids to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addDids")
    public fun com.google.protobuf.kotlin.DslList<kotlin.String, DidsProxy>.add(value: kotlin.String) {
      _builder.addDids(value)
    }
    /**
     * ```
     * W3C Decentralized Identifiers (DIDs) associated to the user.
     * ```
     *
     * `repeated string dids = 10;`
     * @param value The dids to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignDids")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<kotlin.String, DidsProxy>.plusAssign(value: kotlin.String) {
      add(value)
    }
    /**
     * ```
     * W3C Decentralized Identifiers (DIDs) associated to the user.
     * ```
     *
     * `repeated string dids = 10;`
     * @param values The dids to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllDids")
    public fun com.google.protobuf.kotlin.DslList<kotlin.String, DidsProxy>.addAll(values: kotlin.collections.Iterable<kotlin.String>) {
      _builder.addAllDids(values)
    }
    /**
     * ```
     * W3C Decentralized Identifiers (DIDs) associated to the user.
     * ```
     *
     * `repeated string dids = 10;`
     * @param values The dids to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllDids")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<kotlin.String, DidsProxy>.plusAssign(values: kotlin.collections.Iterable<kotlin.String>) {
      addAll(values)
    }
    /**
     * ```
     * W3C Decentralized Identifiers (DIDs) associated to the user.
     * ```
     *
     * `repeated string dids = 10;`
     * @param index The index to set the value at.
     * @param value The dids to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setDids")
    public operator fun com.google.protobuf.kotlin.DslList<kotlin.String, DidsProxy>.set(index: kotlin.Int, value: kotlin.String) {
      _builder.setDids(index, value)
    }/**
     * ```
     * W3C Decentralized Identifiers (DIDs) associated to the user.
     * ```
     *
     * `repeated string dids = 10;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearDids")
    public fun com.google.protobuf.kotlin.DslList<kotlin.String, DidsProxy>.clear() {
      _builder.clearDids()
    }
    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class ExternalIdsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * External system identifiers associated to the user. Ex UserID from the user's financial institution.
     * ```
     *
     * `map<string, string> external_ids = 11;`
     */
     public val externalIds: com.google.protobuf.kotlin.DslMap<kotlin.String, kotlin.String, ExternalIdsProxy>
      @kotlin.jvm.JvmSynthetic
      @JvmName("getExternalIdsMap")
      get() = com.google.protobuf.kotlin.DslMap(
        _builder.getExternalIdsMap()
      )
    /**
     * ```
     * External system identifiers associated to the user. Ex UserID from the user's financial institution.
     * ```
     *
     * `map<string, string> external_ids = 11;`
     */
    @JvmName("putExternalIds")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, kotlin.String, ExternalIdsProxy>
      .put(key: kotlin.String, value: kotlin.String) {
         _builder.putExternalIds(key, value)
       }
    /**
     * ```
     * External system identifiers associated to the user. Ex UserID from the user's financial institution.
     * ```
     *
     * `map<string, string> external_ids = 11;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("setExternalIds")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslMap<kotlin.String, kotlin.String, ExternalIdsProxy>
      .set(key: kotlin.String, value: kotlin.String) {
         put(key, value)
       }
    /**
     * ```
     * External system identifiers associated to the user. Ex UserID from the user's financial institution.
     * ```
     *
     * `map<string, string> external_ids = 11;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("removeExternalIds")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, kotlin.String, ExternalIdsProxy>
      .remove(key: kotlin.String) {
         _builder.removeExternalIds(key)
       }
    /**
     * ```
     * External system identifiers associated to the user. Ex UserID from the user's financial institution.
     * ```
     *
     * `map<string, string> external_ids = 11;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("putAllExternalIds")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, kotlin.String, ExternalIdsProxy>
      .putAll(map: kotlin.collections.Map<kotlin.String, kotlin.String>) {
         _builder.putAllExternalIds(map)
       }
    /**
     * ```
     * External system identifiers associated to the user. Ex UserID from the user's financial institution.
     * ```
     *
     * `map<string, string> external_ids = 11;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("clearExternalIds")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, kotlin.String, ExternalIdsProxy>
      .clear() {
         _builder.clearExternalIds()
       }

    /**
     * ```
     * Additional data about the user such as account, address, branch, routing info, etc.
     * ```
     *
     * `.google.protobuf.Struct metadata = 12;`
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
     * Additional data about the user such as account, address, branch, routing info, etc.
     * ```
     *
     * `.google.protobuf.Struct metadata = 12;`
     */
    public fun clearMetadata() {
      _builder.clearMetadata()
    }
    /**
     * ```
     * Additional data about the user such as account, address, branch, routing info, etc.
     * ```
     *
     * `.google.protobuf.Struct metadata = 12;`
     * @return Whether the metadata field is set.
     */
    public fun hasMetadata(): kotlin.Boolean {
      return _builder.hasMetadata()
    }
    public val UserKt.Dsl.metadataOrNull: com.google.protobuf.Struct?
      get() = _builder.metadataOrNull

    /**
     * ```
     * Account identifier of the user
     * ```
     *
     * `string account = 13;`
     */
    public var account: kotlin.String
      @JvmName("getAccount")
      get() = _builder.getAccount()
      @JvmName("setAccount")
      set(value) {
        _builder.setAccount(value)
      }
    /**
     * ```
     * Account identifier of the user
     * ```
     *
     * `string account = 13;`
     */
    public fun clearAccount() {
      _builder.clearAccount()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.User.copy(block: `user_api.v1`.UserKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.User =
  `user_api.v1`.UserKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val user_api.v1.UserOuterClass.UserOrBuilder.metadataOrNull: com.google.protobuf.Struct?
  get() = if (hasMetadata()) getMetadata() else null

