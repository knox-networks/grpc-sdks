// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: webhook_api/v1/webhook.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package webhook_api.v1;

@kotlin.jvm.JvmName("-initializelistWebhooksResponse")
public inline fun listWebhooksResponse(block: webhook_api.v1.ListWebhooksResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListWebhooksResponse =
  webhook_api.v1.ListWebhooksResponseKt.Dsl._create(webhook_api.v1.WebhookOuterClass.ListWebhooksResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "webhooks": [
 * {
 * "id": "ID1",
 * "owner_id": "eyJwtHeader.eyJwtPayload.Secret1",
 * "name": "NAME1",
 * "webhook_url": "WEBHOOK_URL1",
 * "supported_events": [
 * 0,
 * 1
 * ],
 * "delivery_method": 0
 * },
 * {
 * "id": "ID2",
 * "owner_id": "eyJwtHeader.eyJwtPayload.Secret2",
 * "name": "NAME2",
 * "webhook_url": "WEBHOOK_URL2",
 * "supported_events": [
 * 0,
 * 1
 * ],
 * "delivery_method": 0
 * }
 * ],
 * "total_owned_webhooks": 2
 * }
 * ```
 *
 * Protobuf type `webhook_api.v1.ListWebhooksResponse`
 */
public object ListWebhooksResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: webhook_api.v1.WebhookOuterClass.ListWebhooksResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: webhook_api.v1.WebhookOuterClass.ListWebhooksResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): webhook_api.v1.WebhookOuterClass.ListWebhooksResponse = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class WebhooksProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of webhooks.
     * ```
     *
     * `repeated .webhook_api.v1.Webhook webhooks = 1;`
     */
     public val webhooks: com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.Webhook, WebhooksProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getWebhooksList()
      )
    /**
     * ```
     * List of webhooks.
     * ```
     *
     * `repeated .webhook_api.v1.Webhook webhooks = 1;`
     * @param value The webhooks to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addWebhooks")
    public fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.Webhook, WebhooksProxy>.add(value: webhook_api.v1.WebhookOuterClass.Webhook) {
      _builder.addWebhooks(value)
    }
    /**
     * ```
     * List of webhooks.
     * ```
     *
     * `repeated .webhook_api.v1.Webhook webhooks = 1;`
     * @param value The webhooks to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignWebhooks")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.Webhook, WebhooksProxy>.plusAssign(value: webhook_api.v1.WebhookOuterClass.Webhook) {
      add(value)
    }
    /**
     * ```
     * List of webhooks.
     * ```
     *
     * `repeated .webhook_api.v1.Webhook webhooks = 1;`
     * @param values The webhooks to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllWebhooks")
    public fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.Webhook, WebhooksProxy>.addAll(values: kotlin.collections.Iterable<webhook_api.v1.WebhookOuterClass.Webhook>) {
      _builder.addAllWebhooks(values)
    }
    /**
     * ```
     * List of webhooks.
     * ```
     *
     * `repeated .webhook_api.v1.Webhook webhooks = 1;`
     * @param values The webhooks to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllWebhooks")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.Webhook, WebhooksProxy>.plusAssign(values: kotlin.collections.Iterable<webhook_api.v1.WebhookOuterClass.Webhook>) {
      addAll(values)
    }
    /**
     * ```
     * List of webhooks.
     * ```
     *
     * `repeated .webhook_api.v1.Webhook webhooks = 1;`
     * @param index The index to set the value at.
     * @param value The webhooks to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setWebhooks")
    public operator fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.Webhook, WebhooksProxy>.set(index: kotlin.Int, value: webhook_api.v1.WebhookOuterClass.Webhook) {
      _builder.setWebhooks(index, value)
    }
    /**
     * ```
     * List of webhooks.
     * ```
     *
     * `repeated .webhook_api.v1.Webhook webhooks = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearWebhooks")
    public fun com.google.protobuf.kotlin.DslList<webhook_api.v1.WebhookOuterClass.Webhook, WebhooksProxy>.clear() {
      _builder.clearWebhooks()
    }


    /**
     * ```
     * Total number of webhooks returned.
     * ```
     *
     * `uint32 total_owned_webhooks = 2;`
     */
    public var totalOwnedWebhooks: kotlin.Int
      @JvmName("getTotalOwnedWebhooks")
      get() = _builder.getTotalOwnedWebhooks()
      @JvmName("setTotalOwnedWebhooks")
      set(value) {
        _builder.setTotalOwnedWebhooks(value)
      }
    /**
     * ```
     * Total number of webhooks returned.
     * ```
     *
     * `uint32 total_owned_webhooks = 2;`
     */
    public fun clearTotalOwnedWebhooks() {
      _builder.clearTotalOwnedWebhooks()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun webhook_api.v1.WebhookOuterClass.ListWebhooksResponse.copy(block: `webhook_api.v1`.ListWebhooksResponseKt.Dsl.() -> kotlin.Unit): webhook_api.v1.WebhookOuterClass.ListWebhooksResponse =
  `webhook_api.v1`.ListWebhooksResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()
