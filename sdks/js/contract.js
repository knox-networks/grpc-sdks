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

goog.provide('proto.common.Contract');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Map');
goog.require('jspb.Message');
goog.require('proto.common.Commitment');
goog.require('proto.common.Condition');

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
proto.common.Contract = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.common.Contract.repeatedFields_, null);
};
goog.inherits(proto.common.Contract, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.common.Contract.displayName = 'proto.common.Contract';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.common.Contract.repeatedFields_ = [3,4];



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
proto.common.Contract.prototype.toObject = function(opt_includeInstance) {
  return proto.common.Contract.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.common.Contract} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.Contract.toObject = function(includeInstance, msg) {
  var f, obj = {
    urn: jspb.Message.getFieldWithDefault(msg, 1, ""),
    id: jspb.Message.getFieldWithDefault(msg, 2, ""),
    commitmentsList: jspb.Message.toObjectList(msg.getCommitmentsList(),
    proto.common.Commitment.toObject, includeInstance),
    conditionsList: jspb.Message.toObjectList(msg.getConditionsList(),
    proto.common.Condition.toObject, includeInstance),
    signatoriesMap: (f = msg.getSignatoriesMap()) ? f.toObject(includeInstance, undefined) : [],
    memo: jspb.Message.getFieldWithDefault(msg, 6, "")
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
 * @return {!proto.common.Contract}
 */
proto.common.Contract.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.common.Contract;
  return proto.common.Contract.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.common.Contract} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.common.Contract}
 */
proto.common.Contract.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setUrn(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 3:
      var value = new proto.common.Commitment;
      reader.readMessage(value,proto.common.Commitment.deserializeBinaryFromReader);
      msg.addCommitments(value);
      break;
    case 4:
      var value = new proto.common.Condition;
      reader.readMessage(value,proto.common.Condition.deserializeBinaryFromReader);
      msg.addConditions(value);
      break;
    case 5:
      var value = msg.getSignatoriesMap();
      reader.readMessage(value, function(message, reader) {
        jspb.Map.deserializeBinary(message, reader, jspb.BinaryReader.prototype.readString, jspb.BinaryReader.prototype.readBytes, null, "", "");
         });
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setMemo(value);
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
proto.common.Contract.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.common.Contract.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.common.Contract} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.Contract.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUrn();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getCommitmentsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      3,
      f,
      proto.common.Commitment.serializeBinaryToWriter
    );
  }
  f = message.getConditionsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      4,
      f,
      proto.common.Condition.serializeBinaryToWriter
    );
  }
  f = message.getSignatoriesMap(true);
  if (f && f.getLength() > 0) {
    f.serializeBinary(5, writer, jspb.BinaryWriter.prototype.writeString, jspb.BinaryWriter.prototype.writeBytes);
  }
  f = message.getMemo();
  if (f.length > 0) {
    writer.writeString(
      6,
      f
    );
  }
};


/**
 * optional string urn = 1;
 * @return {string}
 */
proto.common.Contract.prototype.getUrn = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.Contract} returns this
 */
proto.common.Contract.prototype.setUrn = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string id = 2;
 * @return {string}
 */
proto.common.Contract.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.Contract} returns this
 */
proto.common.Contract.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * repeated Commitment commitments = 3;
 * @return {!Array<!proto.common.Commitment>}
 */
proto.common.Contract.prototype.getCommitmentsList = function() {
  return /** @type{!Array<!proto.common.Commitment>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.common.Commitment, 3));
};


/**
 * @param {!Array<!proto.common.Commitment>} value
 * @return {!proto.common.Contract} returns this
*/
proto.common.Contract.prototype.setCommitmentsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 3, value);
};


/**
 * @param {!proto.common.Commitment=} opt_value
 * @param {number=} opt_index
 * @return {!proto.common.Commitment}
 */
proto.common.Contract.prototype.addCommitments = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 3, opt_value, proto.common.Commitment, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.common.Contract} returns this
 */
proto.common.Contract.prototype.clearCommitmentsList = function() {
  return this.setCommitmentsList([]);
};


/**
 * repeated Condition conditions = 4;
 * @return {!Array<!proto.common.Condition>}
 */
proto.common.Contract.prototype.getConditionsList = function() {
  return /** @type{!Array<!proto.common.Condition>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.common.Condition, 4));
};


/**
 * @param {!Array<!proto.common.Condition>} value
 * @return {!proto.common.Contract} returns this
*/
proto.common.Contract.prototype.setConditionsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 4, value);
};


/**
 * @param {!proto.common.Condition=} opt_value
 * @param {number=} opt_index
 * @return {!proto.common.Condition}
 */
proto.common.Contract.prototype.addConditions = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 4, opt_value, proto.common.Condition, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.common.Contract} returns this
 */
proto.common.Contract.prototype.clearConditionsList = function() {
  return this.setConditionsList([]);
};


/**
 * map<string, bytes> signatories = 5;
 * @param {boolean=} opt_noLazyCreate Do not create the map if
 * empty, instead returning `undefined`
 * @return {!jspb.Map<string,!(string|Uint8Array)>}
 */
proto.common.Contract.prototype.getSignatoriesMap = function(opt_noLazyCreate) {
  return /** @type {!jspb.Map<string,!(string|Uint8Array)>} */ (
      jspb.Message.getMapField(this, 5, opt_noLazyCreate,
      null));
};


/**
 * Clears values from the map. The map will be non-null.
 * @return {!proto.common.Contract} returns this
 */
proto.common.Contract.prototype.clearSignatoriesMap = function() {
  this.getSignatoriesMap().clear();
  return this;
};


/**
 * optional string memo = 6;
 * @return {string}
 */
proto.common.Contract.prototype.getMemo = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.Contract} returns this
 */
proto.common.Contract.prototype.setMemo = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
};


