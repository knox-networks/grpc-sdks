// source: wallet_api/v1/wallet.proto
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

goog.provide('proto.wallet_api.v1.ListCredentialsRequest');

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
proto.wallet_api.v1.ListCredentialsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.wallet_api.v1.ListCredentialsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.wallet_api.v1.ListCredentialsRequest.displayName = 'proto.wallet_api.v1.ListCredentialsRequest';
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
proto.wallet_api.v1.ListCredentialsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.wallet_api.v1.ListCredentialsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.wallet_api.v1.ListCredentialsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.ListCredentialsRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    page: jspb.Message.getFieldWithDefault(msg, 1, 0),
    pageSize: jspb.Message.getFieldWithDefault(msg, 2, 0),
    walletId: jspb.Message.getFieldWithDefault(msg, 3, ""),
    apiKey: jspb.Message.getFieldWithDefault(msg, 4, ""),
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
 * @return {!proto.wallet_api.v1.ListCredentialsRequest}
 */
proto.wallet_api.v1.ListCredentialsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.wallet_api.v1.ListCredentialsRequest;
  return proto.wallet_api.v1.ListCredentialsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.wallet_api.v1.ListCredentialsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.wallet_api.v1.ListCredentialsRequest}
 */
proto.wallet_api.v1.ListCredentialsRequest.deserializeBinaryFromReader = function(msg, reader) {
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
      var value = /** @type {string} */ (reader.readString());
      msg.setWalletId(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setApiKey(value);
      break;
    case 5:
      var value = new proto.common.Filter;
      reader.readMessage(value,proto.common.Filter.deserializeBinaryFromReader);
      msg.setFilters(value);
      break;
    case 6:
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
proto.wallet_api.v1.ListCredentialsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.wallet_api.v1.ListCredentialsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.wallet_api.v1.ListCredentialsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.ListCredentialsRequest.serializeBinaryToWriter = function(message, writer) {
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
  f = message.getWalletId();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getApiKey();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getFilters();
  if (f != null) {
    writer.writeMessage(
      5,
      f,
      proto.common.Filter.serializeBinaryToWriter
    );
  }
  f = message.getSortBy();
  if (f != null) {
    writer.writeMessage(
      6,
      f,
      proto.common.SortBy.serializeBinaryToWriter
    );
  }
};


/**
 * optional int32 page = 1;
 * @return {number}
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.wallet_api.v1.ListCredentialsRequest} returns this
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int32 page_size = 2;
 * @return {number}
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.getPageSize = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.wallet_api.v1.ListCredentialsRequest} returns this
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.setPageSize = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional string wallet_id = 3;
 * @return {string}
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.getWalletId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.ListCredentialsRequest} returns this
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.setWalletId = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string api_key = 4;
 * @return {string}
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.getApiKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.ListCredentialsRequest} returns this
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.setApiKey = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional common.Filter filters = 5;
 * @return {?proto.common.Filter}
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.getFilters = function() {
  return /** @type{?proto.common.Filter} */ (
    jspb.Message.getWrapperField(this, proto.common.Filter, 5));
};


/**
 * @param {?proto.common.Filter|undefined} value
 * @return {!proto.wallet_api.v1.ListCredentialsRequest} returns this
*/
proto.wallet_api.v1.ListCredentialsRequest.prototype.setFilters = function(value) {
  return jspb.Message.setWrapperField(this, 5, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.wallet_api.v1.ListCredentialsRequest} returns this
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.clearFilters = function() {
  return this.setFilters(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.hasFilters = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional common.SortBy sort_by = 6;
 * @return {?proto.common.SortBy}
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.getSortBy = function() {
  return /** @type{?proto.common.SortBy} */ (
    jspb.Message.getWrapperField(this, proto.common.SortBy, 6));
};


/**
 * @param {?proto.common.SortBy|undefined} value
 * @return {!proto.wallet_api.v1.ListCredentialsRequest} returns this
*/
proto.wallet_api.v1.ListCredentialsRequest.prototype.setSortBy = function(value) {
  return jspb.Message.setWrapperField(this, 6, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.wallet_api.v1.ListCredentialsRequest} returns this
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.clearSortBy = function() {
  return this.setSortBy(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.wallet_api.v1.ListCredentialsRequest.prototype.hasSortBy = function() {
  return jspb.Message.getField(this, 6) != null;
};


