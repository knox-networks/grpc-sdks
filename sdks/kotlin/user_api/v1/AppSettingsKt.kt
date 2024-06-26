// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_api/v1/user.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package user_api.v1;

@kotlin.jvm.JvmName("-initializeappSettings")
public inline fun appSettings(block: user_api.v1.AppSettingsKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.AppSettings =
  user_api.v1.AppSettingsKt.Dsl._create(user_api.v1.UserOuterClass.AppSettings.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "authn_providers": [
 * {
 * "provider": 0, 
 * "enabled": true
 * }, 
 * {
 * "provider": 1, 
 * "enabled": false
 * }
 * ]
 * }
 * ```
 *
 * Protobuf type `user_api.v1.AppSettings`
 */
public object AppSettingsKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: user_api.v1.UserOuterClass.AppSettings.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: user_api.v1.UserOuterClass.AppSettings.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): user_api.v1.UserOuterClass.AppSettings = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class AuthnProvidersProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * Supported identity providers used for authentication.
     * ```
     *
     * `repeated .user_api.v1.AuthnProvider authn_providers = 1;`
     */
     public val authnProviders: com.google.protobuf.kotlin.DslList<user_api.v1.UserOuterClass.AuthnProvider, AuthnProvidersProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getAuthnProvidersList()
      )
    /**
     * ```
     * Supported identity providers used for authentication.
     * ```
     *
     * `repeated .user_api.v1.AuthnProvider authn_providers = 1;`
     * @param value The authnProviders to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAuthnProviders")
    public fun com.google.protobuf.kotlin.DslList<user_api.v1.UserOuterClass.AuthnProvider, AuthnProvidersProxy>.add(value: user_api.v1.UserOuterClass.AuthnProvider) {
      _builder.addAuthnProviders(value)
    }
    /**
     * ```
     * Supported identity providers used for authentication.
     * ```
     *
     * `repeated .user_api.v1.AuthnProvider authn_providers = 1;`
     * @param value The authnProviders to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAuthnProviders")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<user_api.v1.UserOuterClass.AuthnProvider, AuthnProvidersProxy>.plusAssign(value: user_api.v1.UserOuterClass.AuthnProvider) {
      add(value)
    }
    /**
     * ```
     * Supported identity providers used for authentication.
     * ```
     *
     * `repeated .user_api.v1.AuthnProvider authn_providers = 1;`
     * @param values The authnProviders to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllAuthnProviders")
    public fun com.google.protobuf.kotlin.DslList<user_api.v1.UserOuterClass.AuthnProvider, AuthnProvidersProxy>.addAll(values: kotlin.collections.Iterable<user_api.v1.UserOuterClass.AuthnProvider>) {
      _builder.addAllAuthnProviders(values)
    }
    /**
     * ```
     * Supported identity providers used for authentication.
     * ```
     *
     * `repeated .user_api.v1.AuthnProvider authn_providers = 1;`
     * @param values The authnProviders to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllAuthnProviders")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<user_api.v1.UserOuterClass.AuthnProvider, AuthnProvidersProxy>.plusAssign(values: kotlin.collections.Iterable<user_api.v1.UserOuterClass.AuthnProvider>) {
      addAll(values)
    }
    /**
     * ```
     * Supported identity providers used for authentication.
     * ```
     *
     * `repeated .user_api.v1.AuthnProvider authn_providers = 1;`
     * @param index The index to set the value at.
     * @param value The authnProviders to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setAuthnProviders")
    public operator fun com.google.protobuf.kotlin.DslList<user_api.v1.UserOuterClass.AuthnProvider, AuthnProvidersProxy>.set(index: kotlin.Int, value: user_api.v1.UserOuterClass.AuthnProvider) {
      _builder.setAuthnProviders(index, value)
    }
    /**
     * ```
     * Supported identity providers used for authentication.
     * ```
     *
     * `repeated .user_api.v1.AuthnProvider authn_providers = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearAuthnProviders")
    public fun com.google.protobuf.kotlin.DslList<user_api.v1.UserOuterClass.AuthnProvider, AuthnProvidersProxy>.clear() {
      _builder.clearAuthnProviders()
    }

  }
}
@kotlin.jvm.JvmSynthetic
public inline fun user_api.v1.UserOuterClass.AppSettings.copy(block: `user_api.v1`.AppSettingsKt.Dsl.() -> kotlin.Unit): user_api.v1.UserOuterClass.AppSettings =
  `user_api.v1`.AppSettingsKt.Dsl._create(this.toBuilder()).apply { block() }._build()

