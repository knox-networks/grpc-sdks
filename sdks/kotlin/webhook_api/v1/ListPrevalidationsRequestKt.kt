// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializelistPrevalidationsRequest")
public inline fun listPrevalidationsRequest(block: webhook_api.v1.ListPrevalidationsRequestKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListPrevalidationsRequest =
  webhook_api.v1.ListPrevalidationsRequestKt.Dsl._create(webhook_api.v1.WebhookOuterClass.ListPrevalidationsRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "api_key": "API_KEY",
 * "page": 0,
 * "page_size": 10000,
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
 * Protobuf type `webhook_api.v1.ListPrevalidationsRequest`
 */
public object ListPrevalidationsRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.ListPrevalidationsRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.ListPrevalidationsRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.ListPrevalidationsRequest = _builder.build()

    /**
     * ```
     * API Key as a JWT
     * ```
     *
     * `string api_key = 1;`
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
     * API Key as a JWT
     * ```
     *
     * `string api_key = 1;`
     */
    public fun clearApiKey() {
      _builder.clearApiKey()
    }

    /**
     * ```
     * Page number for pagination.
     * ```
     *
     * `int32 page = 2;`
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
     * `int32 page = 2;`
     */
    public fun clearPage() {
      _builder.clearPage()
    }

    /**
     * ```
     * Size of the page for pagination.
     * ```
     *
     * `int32 page_size = 3;`
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
     * `int32 page_size = 3;`
     */
    public fun clearPageSize() {
      _builder.clearPageSize()
    }

    /**
     * ```
     * Potential filters to apply to the list of prevalidations.
     * ```
     *
     * `.common.Filter filters = 4;`
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
     * Potential filters to apply to the list of prevalidations.
     * ```
     *
     * `.common.Filter filters = 4;`
     */
    public fun clearFilters() {
      _builder.clearFilters()
    }
    /**
     * ```
     * Potential filters to apply to the list of prevalidations.
     * ```
     *
     * `.common.Filter filters = 4;`
     * @return Whether the filters field is set.
     */
    public fun hasFilters(): kotlin.Boolean {
      return _builder.hasFilters()
    }
    public val ListPrevalidationsRequestKt.Dsl.filtersOrNull: common.Common.Filter?
      get() = _builder.filtersOrNull

    /**
     * ```
     * Potential sort to apply to the list of prevalidations.
     * ```
     *
     * `.common.SortBy sort_by = 5;`
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
     * Potential sort to apply to the list of prevalidations.
     * ```
     *
     * `.common.SortBy sort_by = 5;`
     */
    public fun clearSortBy() {
      _builder.clearSortBy()
    }
    /**
     * ```
     * Potential sort to apply to the list of prevalidations.
     * ```
     *
     * `.common.SortBy sort_by = 5;`
     * @return Whether the sortBy field is set.
     */
    public fun hasSortBy(): kotlin.Boolean {
      return _builder.hasSortBy()
    }
    public val ListPrevalidationsRequestKt.Dsl.sortByOrNull: common.Common.SortBy?
      get() = _builder.sortByOrNull
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.ListPrevalidationsRequest.copy(block: `webhook_api.v1`.ListPrevalidationsRequestKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListPrevalidationsRequest =
  `webhook_api.v1`.ListPrevalidationsRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val webhook_api.v1.WebhookOuterClass.ListPrevalidationsRequestOrBuilder.filtersOrNull: common.Common.Filter?
  get() = if (hasFilters()) getFilters() else null

public val webhook_api.v1.WebhookOuterClass.ListPrevalidationsRequestOrBuilder.sortByOrNull: common.Common.SortBy?
  get() = if (hasSortBy()) getSortBy() else null

