// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializelistTransactionsRequest")
public inline fun listTransactionsRequest(block: wallet_api.v1.ListTransactionsRequestKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListTransactionsRequest =
  wallet_api.v1.ListTransactionsRequestKt.Dsl._create(wallet_api.v1.WalletOuterClass.ListTransactionsRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "wallet_id": "zWalletIDMultibase58Encoded",
 * "api_key": "API_KEY",
 * "filters": {
 * "filters": [
 * {
 * "field": "FIELD1",
 * "operator": 0,
 * "value": {
 * "str_value": "STRINGVAL"
 * }
 * },
 * {
 * "field": "FIELD2",
 * "operator": 0,
 * "value": {
 * "int_value": 10
 * }
 * },
 * {
 * "field": "FIELD3",
 * "operator": 0,
 * "value": {
 * "bool_value": false
 * }
 * }
 * ],
 * "operator": 0
 * },
 * "page": 0,
 * "page_size": 10000,
 * "sort_by": {
 * "items": [
 * {
 * "field": "FIELD1",
 * "direction": 0
 * },
 * {
 * "field": "FIELD2",
 * "direction": 0
 * }
 * ]
 * }
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.ListTransactionsRequest`
 */
public object ListTransactionsRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.ListTransactionsRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.ListTransactionsRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.ListTransactionsRequest = _builder.build()

    /**
     * ```
     * Wallet_id as UUID or public_key as multibase.
     * ```
     *
     * `string wallet_id = 1;`
     */
    public var walletId: kotlin.String
      @JvmName("getWalletId")
      get() = _builder.getWalletId()
      @JvmName("setWalletId")
      set(value) {
        _builder.setWalletId(value)
      }
    /**
     * ```
     * Wallet_id as UUID or public_key as multibase.
     * ```
     *
     * `string wallet_id = 1;`
     */
    public fun clearWalletId() {
      _builder.clearWalletId()
    }

    /**
     * ```
     * API key given to the wallet owner at the time of owner creation.
     * ```
     *
     * `string api_key = 2;`
     */
    public var apiKey: kotlin.String
      @JvmName("getApiKey")
      get() = _builder.getApiKey()
      @JvmName("setApiKey")
      set(value) {
        _builder.setApiKey(value)
      }
    /**
     * ```
     * API key given to the wallet owner at the time of owner creation.
     * ```
     *
     * `string api_key = 2;`
     */
    public fun clearApiKey() {
      _builder.clearApiKey()
    }

    /**
     * ```
     * Filters applied to list of transactions.
     * ```
     *
     * `.common.Filter filters = 3;`
     */
    public var filters: common.Common.Filter
      @JvmName("getFilters")
      get() = _builder.getFilters()
      @JvmName("setFilters")
      set(value) {
        _builder.setFilters(value)
      }
    /**
     * ```
     * Filters applied to list of transactions.
     * ```
     *
     * `.common.Filter filters = 3;`
     */
    public fun clearFilters() {
      _builder.clearFilters()
    }
    /**
     * ```
     * Filters applied to list of transactions.
     * ```
     *
     * `.common.Filter filters = 3;`
     * @return Whether the filters field is set.
     */
    public fun hasFilters(): kotlin.Boolean {
      return _builder.hasFilters()
    }
    public val ListTransactionsRequestKt.Dsl.filtersOrNull: common.Common.Filter?
      get() = _builder.filtersOrNull

    /**
     * ```
     * Page number for pagination.
     * ```
     *
     * `int32 page = 4;`
     */
    public var page: kotlin.Int
      @JvmName("getPage")
      get() = _builder.getPage()
      @JvmName("setPage")
      set(value) {
        _builder.setPage(value)
      }
    /**
     * ```
     * Page number for pagination.
     * ```
     *
     * `int32 page = 4;`
     */
    public fun clearPage() {
      _builder.clearPage()
    }

    /**
     * ```
     * Size of the page for pagination.
     * ```
     *
     * `int32 page_size = 5;`
     */
    public var pageSize: kotlin.Int
      @JvmName("getPageSize")
      get() = _builder.getPageSize()
      @JvmName("setPageSize")
      set(value) {
        _builder.setPageSize(value)
      }
    /**
     * ```
     * Size of the page for pagination.
     * ```
     *
     * `int32 page_size = 5;`
     */
    public fun clearPageSize() {
      _builder.clearPageSize()
    }

    /**
     * ```
     * Sort By
     * ```
     *
     * `.common.SortBy sort_by = 6;`
     */
    public var sortBy: common.Common.SortBy
      @JvmName("getSortBy")
      get() = _builder.getSortBy()
      @JvmName("setSortBy")
      set(value) {
        _builder.setSortBy(value)
      }
    /**
     * ```
     * Sort By
     * ```
     *
     * `.common.SortBy sort_by = 6;`
     */
    public fun clearSortBy() {
      _builder.clearSortBy()
    }
    /**
     * ```
     * Sort By
     * ```
     *
     * `.common.SortBy sort_by = 6;`
     * @return Whether the sortBy field is set.
     */
    public fun hasSortBy(): kotlin.Boolean {
      return _builder.hasSortBy()
    }
    public val ListTransactionsRequestKt.Dsl.sortByOrNull: common.Common.SortBy?
      get() = _builder.sortByOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.ListTransactionsRequest.copy(block: `wallet_api.v1`.ListTransactionsRequestKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListTransactionsRequest =
  `wallet_api.v1`.ListTransactionsRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val wallet_api.v1.WalletOuterClass.ListTransactionsRequestOrBuilder.filtersOrNull: common.Common.Filter?
  get() = if (hasFilters()) getFilters() else null

public val wallet_api.v1.WalletOuterClass.ListTransactionsRequestOrBuilder.sortByOrNull: common.Common.SortBy?
  get() = if (hasSortBy()) getSortBy() else null

