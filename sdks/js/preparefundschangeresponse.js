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

goog.provide('proto.wallet_api.v1.PrepareFundsChangeResponse');
goog.provide('proto.wallet_api.v1.PrepareFundsChangeResponse.Status');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');

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
proto.wallet_api.v1.PrepareFundsChangeResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.wallet_api.v1.PrepareFundsChangeResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.wallet_api.v1.PrepareFundsChangeResponse.displayName = 'proto.wallet_api.v1.PrepareFundsChangeResponse';
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
proto.wallet_api.v1.PrepareFundsChangeResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.wallet_api.v1.PrepareFundsChangeResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.wallet_api.v1.PrepareFundsChangeResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.PrepareFundsChangeResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    status: jspb.Message.getFieldWithDefault(msg, 1, 0),
    uetr: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.wallet_api.v1.PrepareFundsChangeResponse}
 */
proto.wallet_api.v1.PrepareFundsChangeResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.wallet_api.v1.PrepareFundsChangeResponse;
  return proto.wallet_api.v1.PrepareFundsChangeResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.wallet_api.v1.PrepareFundsChangeResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.wallet_api.v1.PrepareFundsChangeResponse}
 */
proto.wallet_api.v1.PrepareFundsChangeResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.wallet_api.v1.PrepareFundsChangeResponse.Status} */ (reader.readEnum());
      msg.setStatus(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setUetr(value);
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
proto.wallet_api.v1.PrepareFundsChangeResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.wallet_api.v1.PrepareFundsChangeResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.wallet_api.v1.PrepareFundsChangeResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.wallet_api.v1.PrepareFundsChangeResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getUetr();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * @enum {number}
 */
proto.wallet_api.v1.PrepareFundsChangeResponse.Status = {
  STATUS_UNSPECIFIED: 0,
  STATUS_SUCCESS: 1,
  STATUS_INSUFFICIENT_FUNDS: 2,
  STATUS_FUNDS_PRESENT_NOOP: 3
};

/**
 * optional Status status = 1;
 * @return {!proto.wallet_api.v1.PrepareFundsChangeResponse.Status}
 */
proto.wallet_api.v1.PrepareFundsChangeResponse.prototype.getStatus = function() {
  return /** @type {!proto.wallet_api.v1.PrepareFundsChangeResponse.Status} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.wallet_api.v1.PrepareFundsChangeResponse.Status} value
 * @return {!proto.wallet_api.v1.PrepareFundsChangeResponse} returns this
 */
proto.wallet_api.v1.PrepareFundsChangeResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional string uetr = 2;
 * @return {string}
 */
proto.wallet_api.v1.PrepareFundsChangeResponse.prototype.getUetr = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.wallet_api.v1.PrepareFundsChangeResponse} returns this
 */
proto.wallet_api.v1.PrepareFundsChangeResponse.prototype.setUetr = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


