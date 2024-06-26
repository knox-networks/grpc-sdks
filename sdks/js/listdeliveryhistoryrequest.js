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

goog.provide('proto.webhook_api.v1.ListDeliveryHistoryRequest');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.common.Filter');
goog.require('proto.common.SortBy');

/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.webhook_api.v1.ListDeliveryHistoryRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.webhook_api.v1.ListDeliveryHistoryRequest.displayName = 'proto.webhook_api.v1.ListDeliveryHistoryRequest';
}



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.webhook_api.v1.ListDeliveryHistoryRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.webhook_api.v1.ListDeliveryHistoryRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    apiKey: jspb.Message.getFieldWithDefault(msg, 1, ""),
    page: jspb.Message.getFieldWithDefault(msg, 2, 0),
    pageSize: jspb.Message.getFieldWithDefault(msg, 3, 0),
    filters: (f = msg.getFilters()) && proto.common.Filter.toObject(includeInstance, f),
    sortBy: (f = msg.getSortBy()) && proto.common.SortBy.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.webhook_api.v1.ListDeliveryHistoryRequest}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.webhook_api.v1.ListDeliveryHistoryRequest;
  return proto.webhook_api.v1.ListDeliveryHistoryRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.webhook_api.v1.ListDeliveryHistoryRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.webhook_api.v1.ListDeliveryHistoryRequest}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApiKey(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setPage(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setPageSize(value);
      break;
    case 4:
      var value = new proto.common.Filter;
      reader.readMessage(value,proto.common.Filter.deserializeBinaryFromReader);
      msg.setFilters(value);
      break;
    case 5:
      var value = new proto.common.SortBy;
      reader.readMessage(value,proto.common.SortBy.deserializeBinaryFromReader);
      msg.setSortBy(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.webhook_api.v1.ListDeliveryHistoryRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.webhook_api.v1.ListDeliveryHistoryRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApiKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt32(
      2,
      f
    );
  }
  f = message.getPageSize();
  if (f !== 0) {
    writer.writeInt32(
      3,
      f
    );
  }
  f = message.getFilters();
  if (f != null) {
    writer.writeMessage(
      4,
      f,
      proto.common.Filter.serializeBinaryToWriter
    );
  }
  f = message.getSortBy();
  if (f != null) {
    writer.writeMessage(
      5,
      f,
      proto.common.SortBy.serializeBinaryToWriter
    );
  }
};


/**
 * optional string api_key = 1;
 * @return {string}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.getApiKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.webhook_api.v1.ListDeliveryHistoryRequest} returns this
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.setApiKey = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int32 page = 2;
 * @return {number}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.webhook_api.v1.ListDeliveryHistoryRequest} returns this
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int32 page_size = 3;
 * @return {number}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.getPageSize = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.webhook_api.v1.ListDeliveryHistoryRequest} returns this
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.setPageSize = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional common.Filter filters = 4;
 * @return {?proto.common.Filter}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.getFilters = function() {
  return /** @type{?proto.common.Filter} */ (
    jspb.Message.getWrapperField(this, proto.common.Filter, 4));
};


/**
 * @param {?proto.common.Filter|undefined} value
 * @return {!proto.webhook_api.v1.ListDeliveryHistoryRequest} returns this
*/
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.setFilters = function(value) {
  return jspb.Message.setWrapperField(this, 4, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.webhook_api.v1.ListDeliveryHistoryRequest} returns this
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.clearFilters = function() {
  return this.setFilters(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.hasFilters = function() {
  return jspb.Message.getField(this, 4) != null;
};


/**
 * optional common.SortBy sort_by = 5;
 * @return {?proto.common.SortBy}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.getSortBy = function() {
  return /** @type{?proto.common.SortBy} */ (
    jspb.Message.getWrapperField(this, proto.common.SortBy, 5));
};


/**
 * @param {?proto.common.SortBy|undefined} value
 * @return {!proto.webhook_api.v1.ListDeliveryHistoryRequest} returns this
*/
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.setSortBy = function(value) {
  return jspb.Message.setWrapperField(this, 5, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.webhook_api.v1.ListDeliveryHistoryRequest} returns this
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.clearSortBy = function() {
  return this.setSortBy(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.webhook_api.v1.ListDeliveryHistoryRequest.prototype.hasSortBy = function() {
  return jspb.Message.getField(this, 5) != null;
};


