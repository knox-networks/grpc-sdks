// source: vc_api/v1/vc.proto
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

goog.provide('proto.vc_api.v1.CreateIssuanceChallengeRequest');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');

goog.forwardDeclare('proto.vc_api.v1.CredentialType');
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
proto.vc_api.v1.CreateIssuanceChallengeRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.vc_api.v1.CreateIssuanceChallengeRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.vc_api.v1.CreateIssuanceChallengeRequest.displayName = 'proto.vc_api.v1.CreateIssuanceChallengeRequest';
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
proto.vc_api.v1.CreateIssuanceChallengeRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.vc_api.v1.CreateIssuanceChallengeRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.vc_api.v1.CreateIssuanceChallengeRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.vc_api.v1.CreateIssuanceChallengeRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    credentialType: jspb.Message.getFieldWithDefault(msg, 1, 0),
    did: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.vc_api.v1.CreateIssuanceChallengeRequest}
 */
proto.vc_api.v1.CreateIssuanceChallengeRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.vc_api.v1.CreateIssuanceChallengeRequest;
  return proto.vc_api.v1.CreateIssuanceChallengeRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.vc_api.v1.CreateIssuanceChallengeRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.vc_api.v1.CreateIssuanceChallengeRequest}
 */
proto.vc_api.v1.CreateIssuanceChallengeRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.vc_api.v1.CredentialType} */ (reader.readEnum());
      msg.setCredentialType(value);
      break;
    case 2:
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
proto.vc_api.v1.CreateIssuanceChallengeRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.vc_api.v1.CreateIssuanceChallengeRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.vc_api.v1.CreateIssuanceChallengeRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.vc_api.v1.CreateIssuanceChallengeRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCredentialType();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getDid();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional CredentialType credential_type = 1;
 * @return {!proto.vc_api.v1.CredentialType}
 */
proto.vc_api.v1.CreateIssuanceChallengeRequest.prototype.getCredentialType = function() {
  return /** @type {!proto.vc_api.v1.CredentialType} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.vc_api.v1.CredentialType} value
 * @return {!proto.vc_api.v1.CreateIssuanceChallengeRequest} returns this
 */
proto.vc_api.v1.CreateIssuanceChallengeRequest.prototype.setCredentialType = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional string did = 2;
 * @return {string}
 */
proto.vc_api.v1.CreateIssuanceChallengeRequest.prototype.getDid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.vc_api.v1.CreateIssuanceChallengeRequest} returns this
 */
proto.vc_api.v1.CreateIssuanceChallengeRequest.prototype.setDid = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};

