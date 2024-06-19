// source: common/v1/common.proto
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

goog.provide('proto.common.DynamicVerifier');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');

goog.forwardDeclare('proto.common.SignatureSystem');
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
proto.common.DynamicVerifier = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.common.DynamicVerifier, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.common.DynamicVerifier.displayName = 'proto.common.DynamicVerifier';
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
proto.common.DynamicVerifier.prototype.toObject = function(opt_includeInstance) {
  return proto.common.DynamicVerifier.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.common.DynamicVerifier} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.DynamicVerifier.toObject = function(includeInstance, msg) {
  var f, obj = {
    signatureSystem: jspb.Message.getFieldWithDefault(msg, 1, 0),
    verifier: msg.getVerifier_asB64()
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
 * @return {!proto.common.DynamicVerifier}
 */
proto.common.DynamicVerifier.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.common.DynamicVerifier;
  return proto.common.DynamicVerifier.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.common.DynamicVerifier} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.common.DynamicVerifier}
 */
proto.common.DynamicVerifier.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.common.SignatureSystem} */ (reader.readEnum());
      msg.setSignatureSystem(value);
      break;
    case 2:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setVerifier(value);
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
proto.common.DynamicVerifier.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.common.DynamicVerifier.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.common.DynamicVerifier} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.DynamicVerifier.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSignatureSystem();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getVerifier_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      2,
      f
    );
  }
};


/**
 * optional SignatureSystem signature_system = 1;
 * @return {!proto.common.SignatureSystem}
 */
proto.common.DynamicVerifier.prototype.getSignatureSystem = function() {
  return /** @type {!proto.common.SignatureSystem} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.common.SignatureSystem} value
 * @return {!proto.common.DynamicVerifier} returns this
 */
proto.common.DynamicVerifier.prototype.setSignatureSystem = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional bytes verifier = 2;
 * @return {string}
 */
proto.common.DynamicVerifier.prototype.getVerifier = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * optional bytes verifier = 2;
 * This is a type-conversion wrapper around `getVerifier()`
 * @return {string}
 */
proto.common.DynamicVerifier.prototype.getVerifier_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getVerifier()));
};


/**
 * optional bytes verifier = 2;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getVerifier()`
 * @return {!Uint8Array}
 */
proto.common.DynamicVerifier.prototype.getVerifier_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getVerifier()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.common.DynamicVerifier} returns this
 */
proto.common.DynamicVerifier.prototype.setVerifier = function(value) {
  return jspb.Message.setProto3BytesField(this, 2, value);
};

