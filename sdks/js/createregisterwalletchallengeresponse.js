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

goog.provide('proto.user_api.v1.CreateRegisterWalletChallengeResponse');
goog.provide('proto.user_api.v1.CreateRegisterWalletChallengeResponse.RegistrationStartCase');

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
proto.user_api.v1.CreateRegisterWalletChallengeResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.user_api.v1.CreateRegisterWalletChallengeResponse.oneofGroups_);
};
goog.inherits(proto.user_api.v1.CreateRegisterWalletChallengeResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.user_api.v1.CreateRegisterWalletChallengeResponse.displayName = 'proto.user_api.v1.CreateRegisterWalletChallengeResponse';
}

/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.oneofGroups_ = [[1,2]];

/**
 * @enum {number}
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.RegistrationStartCase = {
  REGISTRATION_START_NOT_SET: 0,
  NONCE: 1,
  OK: 2
};

/**
 * @return {proto.user_api.v1.CreateRegisterWalletChallengeResponse.RegistrationStartCase}
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.getRegistrationStartCase = function() {
  return /** @type {proto.user_api.v1.CreateRegisterWalletChallengeResponse.RegistrationStartCase} */(jspb.Message.computeOneofCase(this, proto.user_api.v1.CreateRegisterWalletChallengeResponse.oneofGroups_[0]));
};



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
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.user_api.v1.CreateRegisterWalletChallengeResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.user_api.v1.CreateRegisterWalletChallengeResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    nonce: jspb.Message.getFieldWithDefault(msg, 1, ""),
    ok: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.user_api.v1.CreateRegisterWalletChallengeResponse}
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.user_api.v1.CreateRegisterWalletChallengeResponse;
  return proto.user_api.v1.CreateRegisterWalletChallengeResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.user_api.v1.CreateRegisterWalletChallengeResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.user_api.v1.CreateRegisterWalletChallengeResponse}
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setNonce(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setOk(value);
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
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.user_api.v1.CreateRegisterWalletChallengeResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.user_api.v1.CreateRegisterWalletChallengeResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = /** @type {string} */ (jspb.Message.getField(message, 1));
  if (f != null) {
    writer.writeString(
      1,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 2));
  if (f != null) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string nonce = 1;
 * @return {string}
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.getNonce = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.user_api.v1.CreateRegisterWalletChallengeResponse} returns this
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.setNonce = function(value) {
  return jspb.Message.setOneofField(this, 1, proto.user_api.v1.CreateRegisterWalletChallengeResponse.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.user_api.v1.CreateRegisterWalletChallengeResponse} returns this
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.clearNonce = function() {
  return jspb.Message.setOneofField(this, 1, proto.user_api.v1.CreateRegisterWalletChallengeResponse.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.hasNonce = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string ok = 2;
 * @return {string}
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.getOk = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.user_api.v1.CreateRegisterWalletChallengeResponse} returns this
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.setOk = function(value) {
  return jspb.Message.setOneofField(this, 2, proto.user_api.v1.CreateRegisterWalletChallengeResponse.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.user_api.v1.CreateRegisterWalletChallengeResponse} returns this
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.clearOk = function() {
  return jspb.Message.setOneofField(this, 2, proto.user_api.v1.CreateRegisterWalletChallengeResponse.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.user_api.v1.CreateRegisterWalletChallengeResponse.prototype.hasOk = function() {
  return jspb.Message.getField(this, 2) != null;
};


