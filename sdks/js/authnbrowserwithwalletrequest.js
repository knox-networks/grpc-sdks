// source: user_api/v1/user.proto
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

goog.provide('proto.user_api.v1.AuthnBrowserWithWalletRequest');

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
proto.user_api.v1.AuthnBrowserWithWalletRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.user_api.v1.AuthnBrowserWithWalletRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.user_api.v1.AuthnBrowserWithWalletRequest.displayName = 'proto.user_api.v1.AuthnBrowserWithWalletRequest';
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
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.user_api.v1.AuthnBrowserWithWalletRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.user_api.v1.AuthnBrowserWithWalletRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    signature: msg.getSignature_asB64(),
    nonce: jspb.Message.getFieldWithDefault(msg, 2, ""),
    did: jspb.Message.getFieldWithDefault(msg, 3, "")
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
 * @return {!proto.user_api.v1.AuthnBrowserWithWalletRequest}
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.user_api.v1.AuthnBrowserWithWalletRequest;
  return proto.user_api.v1.AuthnBrowserWithWalletRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.user_api.v1.AuthnBrowserWithWalletRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.user_api.v1.AuthnBrowserWithWalletRequest}
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setSignature(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setNonce(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setDid(value);
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
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.user_api.v1.AuthnBrowserWithWalletRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.user_api.v1.AuthnBrowserWithWalletRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSignature_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      1,
      f
    );
  }
  f = message.getNonce();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getDid();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
};


/**
 * optional bytes signature = 1;
 * @return {string}
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.getSignature = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * optional bytes signature = 1;
 * This is a type-conversion wrapper around `getSignature()`
 * @return {string}
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.getSignature_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getSignature()));
};


/**
 * optional bytes signature = 1;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getSignature()`
 * @return {!Uint8Array}
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.getSignature_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getSignature()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.user_api.v1.AuthnBrowserWithWalletRequest} returns this
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.setSignature = function(value) {
  return jspb.Message.setProto3BytesField(this, 1, value);
};


/**
 * optional string nonce = 2;
 * @return {string}
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.getNonce = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.user_api.v1.AuthnBrowserWithWalletRequest} returns this
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.setNonce = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string did = 3;
 * @return {string}
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.getDid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.user_api.v1.AuthnBrowserWithWalletRequest} returns this
 */
proto.user_api.v1.AuthnBrowserWithWalletRequest.prototype.setDid = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};

