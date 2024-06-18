// source: webhook_api/v1/webhook.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

goog.provide('proto.webhook_api.v1.DeliveryStage');

/**
 * @enum {number}
 */
proto.webhook_api.v1.DeliveryStage = {
  DELIVERY_STAGE_UNSPECIFIED: 0,
  DELIVERY_STAGE_FIRST_RETRY: 1,
  DELIVERY_STAGE_SECOND_RETRY: 2,
  DELIVERY_STAGE_THIRD_RETRY: 3,
  DELIVERY_STAGE_FOURTH_RETRY: 4,
  DELIVERY_STAGE_FIFTH_RETRY: 5,
  DELIVERY_STAGE_FAILED: 6,
  DELIVERY_STAGE_SUCCESS: 7
};

