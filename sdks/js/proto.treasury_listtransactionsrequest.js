// source: treasury/v1/treasury.proto
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

goog.provide('proto.treasury.ListTransactionsRequest');

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
proto.treasury.ListTransactionsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.treasury.ListTransactionsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.treasury.ListTransactionsRequest.displayName = 'proto.treasury.ListTransactionsRequest';
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
proto.treasury.ListTransactionsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.treasury.ListTransactionsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.treasury.ListTransactionsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.treasury.ListTransactionsRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    page: jspb.Message.getFieldWithDefault(msg, 1, 0),
    pageSize: jspb.Message.getFieldWithDefault(msg, 2, 0),
    sortBy: (f = msg.getSortBy()) && proto.common.SortBy.toObject(includeInstance, f),
    filters: (f = msg.getFilters()) && proto.common.Filter.toObject(includeInstance, f)
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
 * @return {!proto.treasury.ListTransactionsRequest}
 */
proto.treasury.ListTransactionsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.treasury.ListTransactionsRequest;
  return proto.treasury.ListTransactionsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.treasury.ListTransactionsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.treasury.ListTransactionsRequest}
 */
proto.treasury.ListTransactionsRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setPage(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setPageSize(value);
      break;
    case 3:
      var value = new proto.common.SortBy;
      reader.readMessage(value,proto.common.SortBy.deserializeBinaryFromReader);
      msg.setSortBy(value);
      break;
    case 4:
      var value = new proto.common.Filter;
      reader.readMessage(value,proto.common.Filter.deserializeBinaryFromReader);
      msg.setFilters(value);
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
proto.treasury.ListTransactionsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.treasury.ListTransactionsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.treasury.ListTransactionsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.treasury.ListTransactionsRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt32(
      1,
      f
    );
  }
  f = message.getPageSize();
  if (f !== 0) {
    writer.writeInt32(
      2,
      f
    );
  }
  f = message.getSortBy();
  if (f != null) {
    writer.writeMessage(
      3,
      f,
      proto.common.SortBy.serializeBinaryToWriter
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
};


/**
 * optional int32 page = 1;
 * @return {number}
 */
proto.treasury.ListTransactionsRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.treasury.ListTransactionsRequest} returns this
 */
proto.treasury.ListTransactionsRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int32 page_size = 2;
 * @return {number}
 */
proto.treasury.ListTransactionsRequest.prototype.getPageSize = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.treasury.ListTransactionsRequest} returns this
 */
proto.treasury.ListTransactionsRequest.prototype.setPageSize = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional common.SortBy sort_by = 3;
 * @return {?proto.common.SortBy}
 */
proto.treasury.ListTransactionsRequest.prototype.getSortBy = function() {
  return /** @type{?proto.common.SortBy} */ (
    jspb.Message.getWrapperField(this, proto.common.SortBy, 3));
};


/**
 * @param {?proto.common.SortBy|undefined} value
 * @return {!proto.treasury.ListTransactionsRequest} returns this
*/
proto.treasury.ListTransactionsRequest.prototype.setSortBy = function(value) {
  return jspb.Message.setWrapperField(this, 3, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.treasury.ListTransactionsRequest} returns this
 */
proto.treasury.ListTransactionsRequest.prototype.clearSortBy = function() {
  return this.setSortBy(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.treasury.ListTransactionsRequest.prototype.hasSortBy = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional common.Filter filters = 4;
 * @return {?proto.common.Filter}
 */
proto.treasury.ListTransactionsRequest.prototype.getFilters = function() {
  return /** @type{?proto.common.Filter} */ (
    jspb.Message.getWrapperField(this, proto.common.Filter, 4));
};


/**
 * @param {?proto.common.Filter|undefined} value
 * @return {!proto.treasury.ListTransactionsRequest} returns this
*/
proto.treasury.ListTransactionsRequest.prototype.setFilters = function(value) {
  return jspb.Message.setWrapperField(this, 4, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.treasury.ListTransactionsRequest} returns this
 */
proto.treasury.ListTransactionsRequest.prototype.clearFilters = function() {
  return this.setFilters(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.treasury.ListTransactionsRequest.prototype.hasFilters = function() {
  return jspb.Message.getField(this, 4) != null;
};


