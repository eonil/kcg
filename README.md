DEPRECATED
==========
DEPRECATED IN FAVOR OF https://github.com/eonil/ridl







KCG
===
Eonil, 2022.

KCG is short for Schema Code-Gen.

This scans a strict subset of OpenAPI 3.0 spec and generates Rust code.




Getting Started
---------------
1. Prepare a OpenAPI 3.0 schema file.
2. Run KCG with it to generate Rust code.

    ```sh
    kcg api1.yaml impl1.rs
    ```

3. It will show errors and unsupported patterns that KCG does not accept.
4. Fix your OpenAPI spec to pass KCG lint.
5. Do it again until you succeed.

Once you succeeded,
- Compile generated Rust code to validate schema statically.
- Read data instances with generated Rust code to validate them dynamically.

For custom type implementations,
- Use prelude inclusion and skipping.

    ```sh
    kcg api1.yaml impl1.rs --include mine1.rs --skip SampleType1 --skip SampleType2
    ```
- Skip generation of certain types and import your own implementations in prelude code for them.







Supported Schema Model
----------------------
Only these things are supported.
- Primitve types. (`bool`, `i32`, `i64`, `f32`, `f64`, `String`)
- New-type. (`type`)
- Enum-type. (`enum`, finite constant set)
- Sum-type. (`enum`, tagged union, type-based discrimination)
- Product-type. (`struct`)

Here are illustrations of OpenAPI 3.0 Schema patterns for each corresponding models.

New-type.
```yaml
openapi: 3.0.1
info:
    title: Swagger Petstore
    version: 1.2.3
paths: {}
components:
    schemas:
        Order: { type: string }
```
```rust
type Order = String;
```

Enum-type.
```yaml
openapi: 3.0.1
info:
    title: Swagger Petstore
    version: 1.2.3
paths: {}
components:
    schemas:
        Fish: 
            type: string
            enum: [Whale, Shrimp]
```
```rust
enum Fish {
    Wahle, 
    Shrimp,
}
```

Sum-type.
```yaml
openapi: 3.0.1
info:
    title: Swagger Petstore
    version: 1.2.3
paths: {}
components:
    schemas:
        Pet: 
            type: object
            oneOf:
                - $ref: '#/components/schemas/Cat'
                - $ref: '#/components/schemas/Dog'
            discriminator:
                propertyName: type
        Cat: { type: string }
        Dog: { type: string }
```
```rust
enum Pet {
    Cat(Cat)
    Dog(Dog)
}
type Cat = String;
type Dog = String;
```

Product-type.
```yaml
openapi: 3.0.1
info:
    title: Swagger Petstore
    version: 1.2.3
paths: {}
components:
    schemas:
        Ship: 
            type: object
            required: [cargo, crews]
            properties: 
                fuel:
                    type: boolean
                cargo:
                    $ref: '#/components/schemas/Cargo'
                crews:
                    type: array 
                    items: 
                        type: string
        Cargo: 
            type: object
            properties: {}
```
```rust
struct Ship {
    fuel: Option<boolean>,
    cargo: Cargo,
    crews: Vec<String>,
}
struct Cargo {
}
```


Custom Implementation Support
-----------------------------
Schema is a declarative representation of data structures.
It's difficult and inefficient to represent everything in declarative form.
You frequently need to define special types with special behaviors.
For that, you can use prelude inclusion and code-gen skippings.

Here's a command example.
```sh
kcg api1.yaml impl1.rs --include mine1.rs --skip SampleType1 --skip SampleType2
```

Here's content of `mine1.rs`
```rust
use super::{SampleType1,SampleType2};
```

Content of `mine1.rs` will be prefixed in generated code file.
So you can import your own implementation into the generated code.

KCG won't generate type definitions for types designated in `--skip` flags.
In this way, you can avoid duplicated type definitions with your custom types.

Run `tests/sample1/test_full_cycle.sh` at package root to see how this works.







Design Choices
--------------
- Accepts a very narrow strict subset of OpenAPI 3.0 schema.
- Prouces working Rust code.
    - Use Rust compiler for static schema validation.
    - Use compiled Rust program for dynamic data instance validation.
- No inline structures. 
    - Not even tuples.
    - Inline structures with named fields are not well supported in Rust.
- Data model primarily modeled after Rust type system.
    - But extremely simplified.
    - Languages with similar type system can easily be supported later.
- First version will focus on simplicity and least maintenance cost.
    - Schema scanner and code-gen copy things everywhere.
    - Produced Rust code also performs a lot of copies.
    - This can be better by providing updated code-gen.




License
-------
Using this code is licensed under "MIT License".
Contributions will also be licensed under same license.
Contributions mean agreement on this licensing terms.
Copyright(c) 2022, Eonil. All rights reserved.