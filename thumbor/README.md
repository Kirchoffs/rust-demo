# Thumbor

## Get Started
```
>> brew install protobuf
```

```
>> cargo add axum anyhow base64 bytes image lazy_static lru percent-encoding photon-rs
>> cargo add prost reqwest serde tokio tower tower-http tracing tracing-subscriber
>> cargo add --build prost-build

>> cargo add axum-macros

>> mkdir src/pb

>> cargo clean
>> cargo build
```

```
>> cargo build --release
>> RUST_LOG=info target/release/thumbor
```

```
>> tokei src/**/*.rs
```

## Rust Notes
### From & TryFrom
In Rust, both From and TryFrom are traits that define a conversion mechanism between two types. However, there is a fundamental difference between them, which is related to the possibility of the conversion to fail.

The From trait defines a simple, infallible conversion from one type to another. It is used when you know that the conversion can always succeed, and you want to provide a convenient way to convert between types. The From trait is implemented for a wide variety of Rust types, and it is used extensively in the standard library.

On the other hand, the TryFrom trait defines a fallible conversion from one type to another. It is used when the conversion might fail, and you want to handle the error case explicitly.

### Double Colon
```
#[derive(Clone, PartialEq, ::prost::Message)]
```
In Rust, the double colon :: is used to access items in a module or to access associated items (i.e. traits or types) of a type.

In the example you provided, prost is a crate (i.e. a library) that defines a module called prost. The double colon :: is used to access the Message type that is defined in the prost module.

The prefix :: before prost in ::prost::Message means that prost is a top-level module in the crate, rather than a submodule of the current module. This is equivalent to writing crate::prost::Message, where crate refers to the current crate.

### Path

```
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\nspec: {:#?}", url, spec))
}
```

`Path` attribute is often used in web frameworks to extract parameters from the URL path.  
`Path<Params>` specifies that the argument should be of type Path with a generic parameter Params.

`Params { spec, url }` is a destructuring pattern that extracts two fields from the Params struct: spec and url.

`Path(Params { spec, url })` applies the `Path` attribute to the `Params { spec, url }` pattern to extract the parameters from the URL path.

### Difference between std::sync::Mutex and tokio::sync::Mutex


## Other Notes
### ProtoBuf
<b>1. Repeated</b>

```
message ImageSpec { repeated Spec specs = 1; }
```
In the provided message definition, the field specs is marked as repeated. This means that a message of type ImageSpec can contain zero or more instances of the Spec message type, with each instance being identified by a unique index.

<b>2. Value</b>

Enum can have 0 value:
```
enum ResizeType {
    NORMAL = 0;
    SEAM_CARVE = 1;
}
```
While message cannot:
```
message Crop {
    uint32 x1 = 1;
    uint32 y1 = 2;
    uint32 x2 = 3;
    uint32 y2 = 4;
}
```

<b>3. Oneof</b>

```
message Spec {
    oneof data {
        Resize resize = 1;
        Crop crop = 2;
        Flipv flipv = 3;
        Fliph fliph = 4;
        Contrast contrast = 5;
        Filter filter = 6;
        Watermark watermark = 7;
    }
}
```
In Protocol Buffers, the oneof keyword is used to define a union of mutually exclusive fields within a message.

### Image Processing
<b>1. Seam Carving</b>

Seam carving is a content-aware image resizing technique that allows images to be resized in a way that preserves important features and aspect ratios. The idea behind seam carving is to remove or add pixels along a path of least energy in an image, such that the aspect ratio of the image is preserved and important features are not distorted.

It functions by establishing a number of seams (paths of least importance) in an image and automatically removes seams to reduce image size or inserts seams to extend it. Seam carving also allows manually defining areas in which pixels may not be modified, and features the ability to remove whole objects from photographs.

> Aspect ratio: The aspect ratio of an image is the ratio of its width to its height, and is expressed with two numbers separated by a colon, such as 16:9

> Seam: In seam carving, a seam is defined as a connected path of pixels that spans the width or height of the image and has minimal energy cost. The energy cost of a seam is defined as the sum of the difference in pixel values along the seam. Seams with lower energy costs are considered to be less important, while seams with higher energy costs are considered to be more important.

<b>2. URL Encoding</b>

URL encoding is a process of converting certain characters or symbols to their corresponding percent-encoded hexadecimal values in a URL (Uniform Resource Locator) or URI (Uniform Resource Identifier). This is necessary because some characters have special meanings in URLs, such as spaces, question marks, ampersands, and slashes. If these characters are not encoded properly, they can cause issues with the URL and the page or resource it is linking to.

Space - The space character is encoded as %20.  
Example: `https://www.example.com/search?q=hello%20world`

Question mark - The question mark character is encoded as %3F.  
Example: `https://www.example.com/search?q=hello%20world%3F`

Ampersand - The ampersand character is encoded as %26.  
Example: `https://www.example.com/search?q=hello%20world%26more`

Slash - The slash character is encoded as %2F.  
Example: `https://www.example.com/files/folder%2Fsubfolder%2Ffile.txt`

Plus sign - The plus sign character is encoded as %2B.  
Example: `https://www.example.com/search?q=hello%2Bworld`
