// source: registry_api/v1/registry.proto
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

goog.provide('proto.registry_api.v1.CreateCredentialIssuerRequest');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.registry_api.v1.CredentialIssuer');

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
proto.registry_api.v1.CreateCredentialIssuerRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.registry_api.v1.CreateCredentialIssuerRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.registry_api.v1.CreateCredentialIssuerRequest.displayName = 'proto.registry_api.v1.CreateCredentialIssuerRequest';
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
proto.registry_api.v1.CreateCredentialIssuerRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.registry_api.v1.CreateCredentialIssuerRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.registry_api.v1.CreateCredentialIssuerRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.registry_api.v1.CreateCredentialIssuerRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    credentialIssuer: (f = msg.getCredentialIssuer()) && proto.registry_api.v1.CredentialIssuer.toObject(includeInstance, f)
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
 * @return {!proto.registry_api.v1.CreateCredentialIssuerRequest}
 */
proto.registry_api.v1.CreateCredentialIssuerRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.registry_api.v1.CreateCredentialIssuerRequest;
  return proto.registry_api.v1.CreateCredentialIssuerRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.registry_api.v1.CreateCredentialIssuerRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.registry_api.v1.CreateCredentialIssuerRequest}
 */
proto.registry_api.v1.CreateCredentialIssuerRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.registry_api.v1.CredentialIssuer;
      reader.readMessage(value,proto.registry_api.v1.CredentialIssuer.deserializeBinaryFromReader);
      msg.setCredentialIssuer(value);
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
proto.registry_api.v1.CreateCredentialIssuerRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.registry_api.v1.CreateCredentialIssuerRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.registry_api.v1.CreateCredentialIssuerRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.registry_api.v1.CreateCredentialIssuerRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCredentialIssuer();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.registry_api.v1.CredentialIssuer.serializeBinaryToWriter
    );
  }
};


/**
 * optional CredentialIssuer credential_issuer = 1;
 * @return {?proto.registry_api.v1.CredentialIssuer}
 */
proto.registry_api.v1.CreateCredentialIssuerRequest.prototype.getCredentialIssuer = function() {
  return /** @type{?proto.registry_api.v1.CredentialIssuer} */ (
    jspb.Message.getWrapperField(this, proto.registry_api.v1.CredentialIssuer, 1));
};


/**
 * @param {?proto.registry_api.v1.CredentialIssuer|undefined} value
 * @return {!proto.registry_api.v1.CreateCredentialIssuerRequest} returns this
*/
proto.registry_api.v1.CreateCredentialIssuerRequest.prototype.setCredentialIssuer = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.registry_api.v1.CreateCredentialIssuerRequest} returns this
 */
proto.registry_api.v1.CreateCredentialIssuerRequest.prototype.clearCredentialIssuer = function() {
  return this.setCredentialIssuer(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.registry_api.v1.CreateCredentialIssuerRequest.prototype.hasCredentialIssuer = function() {
  return jspb.Message.getField(this, 1) != null;
};


