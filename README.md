# actix-helloworld

## What is actix-helloworld
actix-helloworld It is a simple project that shows the advantages of the [Actix] framework, it is a nice 
introduction for who wants to learn [Actix]

[Actix]: https://actix.rs/

## Requirements
* [Cargo]

[Cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

## Endpoints

```json
{
  "status":true,
  "endpoints":[
    {
      "endpoint":"/api",
      "method":"GET",
      "description":"Explanation of all endpoints",
      "parameters":null,
      "errors":null
    },
    {
      "endpoint":"/api/hello-world",
      "method":"GET",
      "description":"Hello world endpoint",
      "parameters":null,
      "errors":null
    },
    {
      "endpoint":"/api/hello",
      "method":"GET",
      "description":"Say hello to user",
      "parameters":[
        {
          "name":"name",
          "description":"Name of user",
          "optional":true
        }
      ],
      "errors":[
        {
          "status":false,
          "code":400,
          "error":"LongUsername",
          "message":"The username should be smaller than 30 character"
        }
      ]
    },
    {
      "endpoint":"/api/hello",
      "method":"GET",
      "description":"Say hello to user",
      "parameters":[
        {
          "name":"name",
          "description":"Name of user",
          "optional":true
        }
      ],
      "errors":[
        {
          "status":false,
          "code":400,
          "error":"LongUsername",
          "message":"The username should be smaller than 30 character"
        }
      ]
    }
  ]
}
```

## License
actix-helloworld license is [MIT], license file [`LICENSE`]

[MIT]: https://opensource.org/licenses/MIT
[`LICENSE`]: LICENSE
