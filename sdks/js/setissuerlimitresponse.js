// source: authority_api/v1/authority.proto
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

goog.provide('proto.authority_api.v1.SetIssuerLimitResponse');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.common.Amount');

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
proto.authority_api.v1.SetIssuerLimitResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.authority_api.v1.SetIssuerLimitResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.authority_api.v1.SetIssuerLimitResponse.displayName = 'proto.authority_api.v1.SetIssuerLimitResponse';
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
proto.authority_api.v1.SetIssuerLimitResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.authority_api.v1.SetIssuerLimitResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.authority_api.v1.SetIssuerLimitResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.authority_api.v1.SetIssuerLimitResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    limit: (f = msg.getLimit()) && proto.common.Amount.toObject(includeInstance, f),
    signature: msg.getSignature_asB64()
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
 * @return {!proto.authority_api.v1.SetIssuerLimitResponse}
 */
proto.authority_api.v1.SetIssuerLimitResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.authority_api.v1.SetIssuerLimitResponse;
  return proto.authority_api.v1.SetIssuerLimitResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.authority_api.v1.SetIssuerLimitResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.authority_api.v1.SetIssuerLimitResponse}
 */
proto.authority_api.v1.SetIssuerLimitResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.common.Amount;
      reader.readMessage(value,proto.common.Amount.deserializeBinaryFromReader);
      msg.setLimit(value);
      break;
    case 2:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setSignature(value);
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
proto.authority_api.v1.SetIssuerLimitResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.authority_api.v1.SetIssuerLimitResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.authority_api.v1.SetIssuerLimitResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.authority_api.v1.SetIssuerLimitResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getLimit();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.common.Amount.serializeBinaryToWriter
    );
  }
  f = message.getSignature_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      2,
      f
    );
  }
};


/**
 * optional common.Amount limit = 1;
 * @return {?proto.common.Amount}
 */
proto.authority_api.v1.SetIssuerLimitResponse.prototype.getLimit = function() {
  return /** @type{?proto.common.Amount} */ (
    jspb.Message.getWrapperField(this, proto.common.Amount, 1));
};


/**
 * @param {?proto.common.Amount|undefined} value
 * @return {!proto.authority_api.v1.SetIssuerLimitResponse} returns this
*/
proto.authority_api.v1.SetIssuerLimitResponse.prototype.setLimit = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.authority_api.v1.SetIssuerLimitResponse} returns this
 */
proto.authority_api.v1.SetIssuerLimitResponse.prototype.clearLimit = function() {
  return this.setLimit(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.authority_api.v1.SetIssuerLimitResponse.prototype.hasLimit = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional bytes signature = 2;
 * @return {string}
 */
proto.authority_api.v1.SetIssuerLimitResponse.prototype.getSignature = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * optional bytes signature = 2;
 * This is a type-conversion wrapper around `getSignature()`
 * @return {string}
 */
proto.authority_api.v1.SetIssuerLimitResponse.prototype.getSignature_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getSignature()));
};


/**
 * optional bytes signature = 2;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getSignature()`
 * @return {!Uint8Array}
 */
proto.authority_api.v1.SetIssuerLimitResponse.prototype.getSignature_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getSignature()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.authority_api.v1.SetIssuerLimitResponse} returns this
 */
proto.authority_api.v1.SetIssuerLimitResponse.prototype.setSignature = function(value) {
  return jspb.Message.setProto3BytesField(this, 2, value);
};

