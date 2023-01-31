// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: authority_api/v1/authority.proto
// </auto-generated>
#pragma warning disable 0414, 1591
#region Designer generated code

using grpc = global::Grpc.Core;

namespace AuthorityApi.V1 {
  /// <summary>
  /// Entity that is the approving monetary authority- Ex Central bank or commercial bank. It can create Issuers with limits, approve additional issuance limits, authorize issuing of digital banknotes, and replace digital banknotes in circulation.
  /// </summary>
  public static partial class Authority
  {
    static readonly string __ServiceName = "authority_api.v1.Authority";

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static void __Helper_SerializeMessage(global::Google.Protobuf.IMessage message, grpc::SerializationContext context)
    {
      #if !GRPC_DISABLE_PROTOBUF_BUFFER_SERIALIZATION
      if (message is global::Google.Protobuf.IBufferMessage)
      {
        context.SetPayloadLength(message.CalculateSize());
        global::Google.Protobuf.MessageExtensions.WriteTo(message, context.GetBufferWriter());
        context.Complete();
        return;
      }
      #endif
      context.Complete(global::Google.Protobuf.MessageExtensions.ToByteArray(message));
    }

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static class __Helper_MessageCache<T>
    {
      public static readonly bool IsBufferMessage = global::System.Reflection.IntrospectionExtensions.GetTypeInfo(typeof(global::Google.Protobuf.IBufferMessage)).IsAssignableFrom(typeof(T));
    }

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static T __Helper_DeserializeMessage<T>(grpc::DeserializationContext context, global::Google.Protobuf.MessageParser<T> parser) where T : global::Google.Protobuf.IMessage<T>
    {
      #if !GRPC_DISABLE_PROTOBUF_BUFFER_SERIALIZATION
      if (__Helper_MessageCache<T>.IsBufferMessage)
      {
        return parser.ParseFrom(context.PayloadAsReadOnlySequence());
      }
      #endif
      return parser.ParseFrom(context.PayloadAsNewBuffer());
    }

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::AuthorityApi.V1.SetIssuerLimitRequest> __Marshaller_authority_api_v1_SetIssuerLimitRequest = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::AuthorityApi.V1.SetIssuerLimitRequest.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::AuthorityApi.V1.SetIssuerLimitResponse> __Marshaller_authority_api_v1_SetIssuerLimitResponse = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::AuthorityApi.V1.SetIssuerLimitResponse.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::AuthorityApi.V1.GetIssuerLimitsRequest> __Marshaller_authority_api_v1_GetIssuerLimitsRequest = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::AuthorityApi.V1.GetIssuerLimitsRequest.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::AuthorityApi.V1.GetIssuerLimitsResponse> __Marshaller_authority_api_v1_GetIssuerLimitsResponse = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::AuthorityApi.V1.GetIssuerLimitsResponse.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::AuthorityApi.V1.AuthorizeRequest> __Marshaller_authority_api_v1_AuthorizeRequest = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::AuthorityApi.V1.AuthorizeRequest.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::AuthorityApi.V1.AuthorizeResponse> __Marshaller_authority_api_v1_AuthorizeResponse = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::AuthorityApi.V1.AuthorizeResponse.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::AuthorityApi.V1.RedeemRequest> __Marshaller_authority_api_v1_RedeemRequest = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::AuthorityApi.V1.RedeemRequest.Parser));
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Marshaller<global::AuthorityApi.V1.RedeemResponse> __Marshaller_authority_api_v1_RedeemResponse = grpc::Marshallers.Create(__Helper_SerializeMessage, context => __Helper_DeserializeMessage(context, global::AuthorityApi.V1.RedeemResponse.Parser));

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Method<global::AuthorityApi.V1.SetIssuerLimitRequest, global::AuthorityApi.V1.SetIssuerLimitResponse> __Method_SetIssuerLimit = new grpc::Method<global::AuthorityApi.V1.SetIssuerLimitRequest, global::AuthorityApi.V1.SetIssuerLimitResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "SetIssuerLimit",
        __Marshaller_authority_api_v1_SetIssuerLimitRequest,
        __Marshaller_authority_api_v1_SetIssuerLimitResponse);

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Method<global::AuthorityApi.V1.GetIssuerLimitsRequest, global::AuthorityApi.V1.GetIssuerLimitsResponse> __Method_GetIssuerLimits = new grpc::Method<global::AuthorityApi.V1.GetIssuerLimitsRequest, global::AuthorityApi.V1.GetIssuerLimitsResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "GetIssuerLimits",
        __Marshaller_authority_api_v1_GetIssuerLimitsRequest,
        __Marshaller_authority_api_v1_GetIssuerLimitsResponse);

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Method<global::AuthorityApi.V1.AuthorizeRequest, global::AuthorityApi.V1.AuthorizeResponse> __Method_Authorize = new grpc::Method<global::AuthorityApi.V1.AuthorizeRequest, global::AuthorityApi.V1.AuthorizeResponse>(
        grpc::MethodType.DuplexStreaming,
        __ServiceName,
        "Authorize",
        __Marshaller_authority_api_v1_AuthorizeRequest,
        __Marshaller_authority_api_v1_AuthorizeResponse);

    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    static readonly grpc::Method<global::AuthorityApi.V1.RedeemRequest, global::AuthorityApi.V1.RedeemResponse> __Method_Redeem = new grpc::Method<global::AuthorityApi.V1.RedeemRequest, global::AuthorityApi.V1.RedeemResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Redeem",
        __Marshaller_authority_api_v1_RedeemRequest,
        __Marshaller_authority_api_v1_RedeemResponse);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::AuthorityApi.V1.AuthorityReflection.Descriptor.Services[0]; }
    }

    /// <summary>Base class for server-side implementations of Authority</summary>
    [grpc::BindServiceMethod(typeof(Authority), "BindService")]
    public abstract partial class AuthorityBase
    {
      /// <summary>
      /// Sets the issuance limit for an Issuer to a specified value and returns the new limit.
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::System.Threading.Tasks.Task<global::AuthorityApi.V1.SetIssuerLimitResponse> SetIssuerLimit(global::AuthorityApi.V1.SetIssuerLimitRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      /// <summary>
      /// Gets the the issuance limits for all the currencies for which the Issuer is allowed to issue digital banknotes.
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::System.Threading.Tasks.Task<global::AuthorityApi.V1.GetIssuerLimitsResponse> GetIssuerLimits(global::AuthorityApi.V1.GetIssuerLimitsRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      /// <summary>
      /// Accepts digital banknotes in a stream, authorizes them and returns authorized digital banknotes in a stream.
      /// </summary>
      /// <param name="requestStream">Used for reading requests from the client.</param>
      /// <param name="responseStream">Used for sending responses back to the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>A task indicating completion of the handler.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::System.Threading.Tasks.Task Authorize(grpc::IAsyncStreamReader<global::AuthorityApi.V1.AuthorizeRequest> requestStream, grpc::IServerStreamWriter<global::AuthorityApi.V1.AuthorizeResponse> responseStream, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      /// <summary>
      /// Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed digital banknote.
      /// </summary>
      /// <param name="request">The request received from the client.</param>
      /// <param name="context">The context of the server-side call handler being invoked.</param>
      /// <returns>The response to send back to the client (wrapped by a task).</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::System.Threading.Tasks.Task<global::AuthorityApi.V1.RedeemResponse> Redeem(global::AuthorityApi.V1.RedeemRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for Authority</summary>
    public partial class AuthorityClient : grpc::ClientBase<AuthorityClient>
    {
      /// <summary>Creates a new client for Authority</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public AuthorityClient(grpc::ChannelBase channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for Authority that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public AuthorityClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      protected AuthorityClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      protected AuthorityClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      /// <summary>
      /// Sets the issuance limit for an Issuer to a specified value and returns the new limit.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::AuthorityApi.V1.SetIssuerLimitResponse SetIssuerLimit(global::AuthorityApi.V1.SetIssuerLimitRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return SetIssuerLimit(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Sets the issuance limit for an Issuer to a specified value and returns the new limit.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::AuthorityApi.V1.SetIssuerLimitResponse SetIssuerLimit(global::AuthorityApi.V1.SetIssuerLimitRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_SetIssuerLimit, null, options, request);
      }
      /// <summary>
      /// Sets the issuance limit for an Issuer to a specified value and returns the new limit.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::AuthorityApi.V1.SetIssuerLimitResponse> SetIssuerLimitAsync(global::AuthorityApi.V1.SetIssuerLimitRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return SetIssuerLimitAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Sets the issuance limit for an Issuer to a specified value and returns the new limit.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::AuthorityApi.V1.SetIssuerLimitResponse> SetIssuerLimitAsync(global::AuthorityApi.V1.SetIssuerLimitRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_SetIssuerLimit, null, options, request);
      }
      /// <summary>
      /// Gets the the issuance limits for all the currencies for which the Issuer is allowed to issue digital banknotes.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::AuthorityApi.V1.GetIssuerLimitsResponse GetIssuerLimits(global::AuthorityApi.V1.GetIssuerLimitsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetIssuerLimits(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Gets the the issuance limits for all the currencies for which the Issuer is allowed to issue digital banknotes.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::AuthorityApi.V1.GetIssuerLimitsResponse GetIssuerLimits(global::AuthorityApi.V1.GetIssuerLimitsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_GetIssuerLimits, null, options, request);
      }
      /// <summary>
      /// Gets the the issuance limits for all the currencies for which the Issuer is allowed to issue digital banknotes.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::AuthorityApi.V1.GetIssuerLimitsResponse> GetIssuerLimitsAsync(global::AuthorityApi.V1.GetIssuerLimitsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetIssuerLimitsAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Gets the the issuance limits for all the currencies for which the Issuer is allowed to issue digital banknotes.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::AuthorityApi.V1.GetIssuerLimitsResponse> GetIssuerLimitsAsync(global::AuthorityApi.V1.GetIssuerLimitsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_GetIssuerLimits, null, options, request);
      }
      /// <summary>
      /// Accepts digital banknotes in a stream, authorizes them and returns authorized digital banknotes in a stream.
      /// </summary>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncDuplexStreamingCall<global::AuthorityApi.V1.AuthorizeRequest, global::AuthorityApi.V1.AuthorizeResponse> Authorize(grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Authorize(new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Accepts digital banknotes in a stream, authorizes them and returns authorized digital banknotes in a stream.
      /// </summary>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncDuplexStreamingCall<global::AuthorityApi.V1.AuthorizeRequest, global::AuthorityApi.V1.AuthorizeResponse> Authorize(grpc::CallOptions options)
      {
        return CallInvoker.AsyncDuplexStreamingCall(__Method_Authorize, null, options);
      }
      /// <summary>
      /// Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed digital banknote.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::AuthorityApi.V1.RedeemResponse Redeem(global::AuthorityApi.V1.RedeemRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Redeem(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed digital banknote.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The response received from the server.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual global::AuthorityApi.V1.RedeemResponse Redeem(global::AuthorityApi.V1.RedeemRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Redeem, null, options, request);
      }
      /// <summary>
      /// Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed digital banknote.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="headers">The initial metadata to send with the call. This parameter is optional.</param>
      /// <param name="deadline">An optional deadline for the call. The call will be cancelled if deadline is hit.</param>
      /// <param name="cancellationToken">An optional token for canceling the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::AuthorityApi.V1.RedeemResponse> RedeemAsync(global::AuthorityApi.V1.RedeemRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return RedeemAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      /// <summary>
      /// Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed digital banknote.
      /// </summary>
      /// <param name="request">The request to send to the server.</param>
      /// <param name="options">The options for the call.</param>
      /// <returns>The call object.</returns>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      public virtual grpc::AsyncUnaryCall<global::AuthorityApi.V1.RedeemResponse> RedeemAsync(global::AuthorityApi.V1.RedeemRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Redeem, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
      protected override AuthorityClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new AuthorityClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    public static grpc::ServerServiceDefinition BindService(AuthorityBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_SetIssuerLimit, serviceImpl.SetIssuerLimit)
          .AddMethod(__Method_GetIssuerLimits, serviceImpl.GetIssuerLimits)
          .AddMethod(__Method_Authorize, serviceImpl.Authorize)
          .AddMethod(__Method_Redeem, serviceImpl.Redeem).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the  service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    [global::System.CodeDom.Compiler.GeneratedCode("grpc_csharp_plugin", null)]
    public static void BindService(grpc::ServiceBinderBase serviceBinder, AuthorityBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_SetIssuerLimit, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::AuthorityApi.V1.SetIssuerLimitRequest, global::AuthorityApi.V1.SetIssuerLimitResponse>(serviceImpl.SetIssuerLimit));
      serviceBinder.AddMethod(__Method_GetIssuerLimits, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::AuthorityApi.V1.GetIssuerLimitsRequest, global::AuthorityApi.V1.GetIssuerLimitsResponse>(serviceImpl.GetIssuerLimits));
      serviceBinder.AddMethod(__Method_Authorize, serviceImpl == null ? null : new grpc::DuplexStreamingServerMethod<global::AuthorityApi.V1.AuthorizeRequest, global::AuthorityApi.V1.AuthorizeResponse>(serviceImpl.Authorize));
      serviceBinder.AddMethod(__Method_Redeem, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::AuthorityApi.V1.RedeemRequest, global::AuthorityApi.V1.RedeemResponse>(serviceImpl.Redeem));
    }

  }
}
#endregion
