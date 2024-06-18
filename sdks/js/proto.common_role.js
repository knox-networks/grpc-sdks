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

goog.provide('proto.common.Role');
goog.provide('proto.common.Role.IssuePermissions');
goog.provide('proto.common.Role.RoleEnumCase');

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
proto.common.Role = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.common.Role.oneofGroups_);
};
goog.inherits(proto.common.Role, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.common.Role.displayName = 'proto.common.Role';
}
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
proto.common.Role.IssuePermissions = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.common.Role.IssuePermissions.repeatedFields_, null);
};
goog.inherits(proto.common.Role.IssuePermissions, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.common.Role.IssuePermissions.displayName = 'proto.common.Role.IssuePermissions';
}

/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.common.Role.oneofGroups_ = [[3,4]];

/**
 * @enum {number}
 */
proto.common.Role.RoleEnumCase = {
  ROLE_ENUM_NOT_SET: 0,
  AUTHORIZED: 3,
  ISSUE_PERMISSIONS: 4
};

/**
 * @return {proto.common.Role.RoleEnumCase}
 */
proto.common.Role.prototype.getRoleEnumCase = function() {
  return /** @type {proto.common.Role.RoleEnumCase} */(jspb.Message.computeOneofCase(this, proto.common.Role.oneofGroups_[0]));
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
proto.common.Role.prototype.toObject = function(opt_includeInstance) {
  return proto.common.Role.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.common.Role} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.Role.toObject = function(includeInstance, msg) {
  var f, obj = {
    verifier: jspb.Message.getFieldWithDefault(msg, 1, ""),
    authorized: jspb.Message.getBooleanFieldWithDefault(msg, 3, false),
    issuePermissions: (f = msg.getIssuePermissions()) && proto.common.Role.IssuePermissions.toObject(includeInstance, f)
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
 * @return {!proto.common.Role}
 */
proto.common.Role.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.common.Role;
  return proto.common.Role.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.common.Role} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.common.Role}
 */
proto.common.Role.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setVerifier(value);
      break;
    case 3:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setAuthorized(value);
      break;
    case 4:
      var value = new proto.common.Role.IssuePermissions;
      reader.readMessage(value,proto.common.Role.IssuePermissions.deserializeBinaryFromReader);
      msg.setIssuePermissions(value);
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
proto.common.Role.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.common.Role.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.common.Role} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.Role.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getVerifier();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = /** @type {boolean} */ (jspb.Message.getField(message, 3));
  if (f != null) {
    writer.writeBool(
      3,
      f
    );
  }
  f = message.getIssuePermissions();
  if (f != null) {
    writer.writeMessage(
      4,
      f,
      proto.common.Role.IssuePermissions.serializeBinaryToWriter
    );
  }
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.common.Role.IssuePermissions.repeatedFields_ = [2];



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
proto.common.Role.IssuePermissions.prototype.toObject = function(opt_includeInstance) {
  return proto.common.Role.IssuePermissions.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.common.Role.IssuePermissions} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.Role.IssuePermissions.toObject = function(includeInstance, msg) {
  var f, obj = {
    innerList: jspb.Message.toObjectList(msg.getInnerList(),
    proto.common.Amount.toObject, includeInstance)
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
 * @return {!proto.common.Role.IssuePermissions}
 */
proto.common.Role.IssuePermissions.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.common.Role.IssuePermissions;
  return proto.common.Role.IssuePermissions.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.common.Role.IssuePermissions} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.common.Role.IssuePermissions}
 */
proto.common.Role.IssuePermissions.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 2:
      var value = new proto.common.Amount;
      reader.readMessage(value,proto.common.Amount.deserializeBinaryFromReader);
      msg.addInner(value);
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
proto.common.Role.IssuePermissions.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.common.Role.IssuePermissions.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.common.Role.IssuePermissions} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.common.Role.IssuePermissions.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getInnerList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.common.Amount.serializeBinaryToWriter
    );
  }
};


/**
 * repeated Amount inner = 2;
 * @return {!Array<!proto.common.Amount>}
 */
proto.common.Role.IssuePermissions.prototype.getInnerList = function() {
  return /** @type{!Array<!proto.common.Amount>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.common.Amount, 2));
};


/**
 * @param {!Array<!proto.common.Amount>} value
 * @return {!proto.common.Role.IssuePermissions} returns this
*/
proto.common.Role.IssuePermissions.prototype.setInnerList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 2, value);
};


/**
 * @param {!proto.common.Amount=} opt_value
 * @param {number=} opt_index
 * @return {!proto.common.Amount}
 */
proto.common.Role.IssuePermissions.prototype.addInner = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 2, opt_value, proto.common.Amount, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.common.Role.IssuePermissions} returns this
 */
proto.common.Role.IssuePermissions.prototype.clearInnerList = function() {
  return this.setInnerList([]);
};


/**
 * optional string verifier = 1;
 * @return {string}
 */
proto.common.Role.prototype.getVerifier = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.common.Role} returns this
 */
proto.common.Role.prototype.setVerifier = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional bool authorized = 3;
 * @return {boolean}
 */
proto.common.Role.prototype.getAuthorized = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 3, false));
};


/**
 * @param {boolean} value
 * @return {!proto.common.Role} returns this
 */
proto.common.Role.prototype.setAuthorized = function(value) {
  return jspb.Message.setOneofField(this, 3, proto.common.Role.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.common.Role} returns this
 */
proto.common.Role.prototype.clearAuthorized = function() {
  return jspb.Message.setOneofField(this, 3, proto.common.Role.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.Role.prototype.hasAuthorized = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional IssuePermissions issue_permissions = 4;
 * @return {?proto.common.Role.IssuePermissions}
 */
proto.common.Role.prototype.getIssuePermissions = function() {
  return /** @type{?proto.common.Role.IssuePermissions} */ (
    jspb.Message.getWrapperField(this, proto.common.Role.IssuePermissions, 4));
};


/**
 * @param {?proto.common.Role.IssuePermissions|undefined} value
 * @return {!proto.common.Role} returns this
*/
proto.common.Role.prototype.setIssuePermissions = function(value) {
  return jspb.Message.setOneofWrapperField(this, 4, proto.common.Role.oneofGroups_[0], value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.common.Role} returns this
 */
proto.common.Role.prototype.clearIssuePermissions = function() {
  return this.setIssuePermissions(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.common.Role.prototype.hasIssuePermissions = function() {
  return jspb.Message.getField(this, 4) != null;
};


