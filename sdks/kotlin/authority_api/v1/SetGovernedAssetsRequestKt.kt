// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: authority_api/v1/authority.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package authority_api.v1;

@kotlin.jvm.JvmName("-initializesetGovernedAssetsRequest")
public inline fun setGovernedAssetsRequest(block: authority_api.v1.SetGovernedAssetsRequestKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.SetGovernedAssetsRequest =
  authority_api.v1.SetGovernedAssetsRequestKt.Dsl._create(authority_api.v1.AuthorityOuterClass.SetGovernedAssetsRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "asset_data": [
 * {
 * "key": "USD",
 * "value": {
 * "denominations": [1, 5, 10, 25, 100, 200, 500, 1000, 2000, 5000, 10000, 50000],
 * "precision": 2
 * }
 * },
 * {
 * "key": "JPY",
 * "value": {
 * "denominations": [1, 5, 10, 50, 100, 500, 1000, 2000, 5000, 10000, 50000, 100000],
 * "precision": 0
 * }
 * }
 * ]
 * }
 * ```
 *
 * Protobuf type `authority_api.v1.SetGovernedAssetsRequest`
 */
public object SetGovernedAssetsRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: authority_api.v1.AuthorityOuterClass.SetGovernedAssetsRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: authority_api.v1.AuthorityOuterClass.SetGovernedAssetsRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): authority_api.v1.AuthorityOuterClass.SetGovernedAssetsRequest = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class AssetDataProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of requested Governed Assets.
     * ```
     *
     * `map<string, .common.AssetDenomination> asset_data = 1;`
     */
     public val assetData: com.google.protobuf.kotlin.DslMap<kotlin.String, common.Common.AssetDenomination, AssetDataProxy>
      @kotlin.jvm.JvmSynthetic
      @JvmName("getAssetDataMap")
      get() = com.google.protobuf.kotlin.DslMap(
        _builder.getAssetDataMap()
      )
    /**
     * ```
     * List of requested Governed Assets.
     * ```
     *
     * `map<string, .common.AssetDenomination> asset_data = 1;`
     */
    @JvmName("putAssetData")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, common.Common.AssetDenomination, AssetDataProxy>
      .put(key: kotlin.String, value: common.Common.AssetDenomination) {
         _builder.putAssetData(key, value)
       }
    /**
     * ```
     * List of requested Governed Assets.
     * ```
     *
     * `map<string, .common.AssetDenomination> asset_data = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("setAssetData")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslMap<kotlin.String, common.Common.AssetDenomination, AssetDataProxy>
      .set(key: kotlin.String, value: common.Common.AssetDenomination) {
         put(key, value)
       }
    /**
     * ```
     * List of requested Governed Assets.
     * ```
     *
     * `map<string, .common.AssetDenomination> asset_data = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("removeAssetData")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, common.Common.AssetDenomination, AssetDataProxy>
      .remove(key: kotlin.String) {
         _builder.removeAssetData(key)
       }
    /**
     * ```
     * List of requested Governed Assets.
     * ```
     *
     * `map<string, .common.AssetDenomination> asset_data = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("putAllAssetData")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, common.Common.AssetDenomination, AssetDataProxy>
      .putAll(map: kotlin.collections.Map<kotlin.String, common.Common.AssetDenomination>) {
         _builder.putAllAssetData(map)
       }
    /**
     * ```
     * List of requested Governed Assets.
     * ```
     *
     * `map<string, .common.AssetDenomination> asset_data = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @JvmName("clearAssetData")
    public fun com.google.protobuf.kotlin.DslMap<kotlin.String, common.Common.AssetDenomination, AssetDataProxy>
      .clear() {
         _builder.clearAssetData()
       }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun authority_api.v1.AuthorityOuterClass.SetGovernedAssetsRequest.copy(block: `authority_api.v1`.SetGovernedAssetsRequestKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.SetGovernedAssetsRequest =
  `authority_api.v1`.SetGovernedAssetsRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

