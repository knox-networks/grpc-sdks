// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: authority_api/v1/authority.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package authority_api.v1;

@kotlin.jvm.JvmName("-initializeaddAssetRequest")
public inline fun addAssetRequest(block: authority_api.v1.AddAssetRequestKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.AddAssetRequest =
  authority_api.v1.AddAssetRequestKt.Dsl._create(authority_api.v1.AuthorityOuterClass.AddAssetRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "currency_code": "USD",
 * "asset_denomination": {
 * "denominations": [
 * 1,
 * 5,
 * 10,
 * 25,
 * 100,
 * 200,
 * 500,
 * 1000,
 * 2000,
 * 5000,
 * 10000,
 * 50000
 * ],
 * "precision": 2
 * }
 * }
 * ```
 *
 * Protobuf type `authority_api.v1.AddAssetRequest`
 */
public object AddAssetRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: authority_api.v1.AuthorityOuterClass.AddAssetRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: authority_api.v1.AuthorityOuterClass.AddAssetRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): authority_api.v1.AuthorityOuterClass.AddAssetRequest = _builder.build()

    /**
     * ```
     * Currency Code
     * ```
     *
     * `string currency_code = 1;`
     */
    public var currencyCode: kotlin.String
      @JvmName("getCurrencyCode")
      get() = _builder.getCurrencyCode()
      @JvmName("setCurrencyCode")
      set(value) {
        _builder.setCurrencyCode(value)
      }
    /**
     * ```
     * Currency Code
     * ```
     *
     * `string currency_code = 1;`
     */
    public fun clearCurrencyCode() {
      _builder.clearCurrencyCode()
    }

    /**
     * ```
     * Asset Denomination
     * ```
     *
     * `.common.AssetDenomination asset_denomination = 2;`
     */
    public var assetDenomination: common.Common.AssetDenomination
      @JvmName("getAssetDenomination")
      get() = _builder.getAssetDenomination()
      @JvmName("setAssetDenomination")
      set(value) {
        _builder.setAssetDenomination(value)
      }
    /**
     * ```
     * Asset Denomination
     * ```
     *
     * `.common.AssetDenomination asset_denomination = 2;`
     */
    public fun clearAssetDenomination() {
      _builder.clearAssetDenomination()
    }
    /**
     * ```
     * Asset Denomination
     * ```
     *
     * `.common.AssetDenomination asset_denomination = 2;`
     * @return Whether the assetDenomination field is set.
     */
    public fun hasAssetDenomination(): kotlin.Boolean {
      return _builder.hasAssetDenomination()
    }
    public val AddAssetRequestKt.Dsl.assetDenominationOrNull: common.Common.AssetDenomination?
      get() = _builder.assetDenominationOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun authority_api.v1.AuthorityOuterClass.AddAssetRequest.copy(block: `authority_api.v1`.AddAssetRequestKt.Dsl.() -> kotlin.Unit): authority_api.v1.AuthorityOuterClass.AddAssetRequest =
  `authority_api.v1`.AddAssetRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val authority_api.v1.AuthorityOuterClass.AddAssetRequestOrBuilder.assetDenominationOrNull: common.Common.AssetDenomination?
  get() = if (hasAssetDenomination()) getAssetDenomination() else null

