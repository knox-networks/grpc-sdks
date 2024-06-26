// source: common/v1/packet.proto
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

goog.provide('proto.common.HashCondition');

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
proto.common.HashCondition = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.common.HashCondition, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.common.HashCondition.displayName = 'proto.common.HashCondition';
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
proto.common.HashCondition.prototype.toObject = function(opt_includeInstance) {
  return proto.common.HashCondition.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.common.HashCondition} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.HashCondition.toObject = function(includeInstance, msg) {
  var f, obj = {
    presenter: jspb.Message.getFieldWithDefault(msg, 1, ""),
    hash: msg.getHash_asB64(),
    algorithmOid: msg.getAlgorithmOid_asB64()
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
 * @return {!proto.common.HashCondition}
 */
proto.common.HashCondition.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.common.HashCondition;
  return proto.common.HashCondition.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.common.HashCondition} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.common.HashCondition}
 */
proto.common.HashCondition.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setPresenter(value);
      break;
    case 2:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setHash(value);
      break;
    case 3:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setAlgorithmOid(value);
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
proto.common.HashCondition.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.common.HashCondition.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.common.HashCondition} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.HashCondition.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPresenter();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getHash_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      2,
      f
    );
  }
  f = message.getAlgorithmOid_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      3,
      f
    );
  }
};


/**
 * optional string presenter = 1;
 * @return {string}
 */
proto.common.HashCondition.prototype.getPresenter = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.HashCondition} returns this
 */
proto.common.HashCondition.prototype.setPresenter = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional bytes hash = 2;
 * @return {string}
 */
proto.common.HashCondition.prototype.getHash = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * optional bytes hash = 2;
 * This is a type-conversion wrapper around `getHash()`
 * @return {string}
 */
proto.common.HashCondition.prototype.getHash_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getHash()));
};


/**
 * optional bytes hash = 2;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getHash()`
 * @return {!Uint8Array}
 */
proto.common.HashCondition.prototype.getHash_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getHash()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.common.HashCondition} returns this
 */
proto.common.HashCondition.prototype.setHash = function(value) {
  return jspb.Message.setProto3BytesField(this, 2, value);
};


/**
 * optional bytes algorithm_oid = 3;
 * @return {string}
 */
proto.common.HashCondition.prototype.getAlgorithmOid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * optional bytes algorithm_oid = 3;
 * This is a type-conversion wrapper around `getAlgorithmOid()`
 * @return {string}
 */
proto.common.HashCondition.prototype.getAlgorithmOid_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getAlgorithmOid()));
};


/**
 * optional bytes algorithm_oid = 3;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getAlgorithmOid()`
 * @return {!Uint8Array}
 */
proto.common.HashCondition.prototype.getAlgorithmOid_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getAlgorithmOid()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.common.HashCondition} returns this
 */
proto.common.HashCondition.prototype.setAlgorithmOid = function(value) {
  return jspb.Message.setProto3BytesField(this, 3, value);
};


