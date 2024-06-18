# Knox Networks gRPC-generated SDKs

Welcome to Knox Network's Generated SDKs. 

# Getting Started

If you have not visited [Knox's Developer Portal](https://dev.knoxnetworks.io/), we strongly recommend you do so in order to better understand how different components within the network interact with one another.
 
Knox uses [gRPC](https://grpc.io) to provide a more efficient API. Unlike a REST API, gRPC APIs require the downloading of specific SDKs in order to interact with the server. Normally, you'd have to download the `.proto` files and generate the code yourself, but you can just download the pre-built SDKs here.

With the SDKs downloaded, you will be able to interact with the Knox API.

# Languages

Everything in the `sdk/` folder is generated automatically from `.proto` files. Currently, the generated SDKs are provided for the following languages in the `sdk` folder:

- [C++](./sdk/cpp)
- [C#/.NET](./sdk/csharp)
- [Go](./sdk/go)
- [Java](./sdk/java)
- [JavaScript](./sdk/js)
- [Kotlin](./sdk/kotlin)
- [Python](./sdk/python)
- [Rust](./sdk/rust) - EXPERIMENTAL
- [TypeScript](./sdk/ts)
