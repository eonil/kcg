KCG
===
Eonil, 2022.

KCG is short for Schema Code-Gen.

This scans a strict subset of OpenAPI 3.0 spec and generates Rust code.




Getting Started
---------------
1. Prepare a OpenAPI 3.0 schema file.
2. Run KCG with it.

        kcg my-api.yaml

3. It will show errors and unsupported patterns that KCG does not accept.
4. Fix your OpenAPI spec to pass KCG lint.
5. Generate Rust code.

        kcg my-api.yaml sample.rs

6. Compile generated Rust code to validate schema statically.
7. read data instances with generated Rust code to validate them dynamically.





Supported Schema Model
----------------------
Only these things are supported.
- Primitve types. (`bool`, `i32`, `i64`, `f32`, `f64`, `String`)
- New-type. (`type`)
- Sum-type. (`enum`)
- Product-type. (`struct`)

Here are illustrations of OpenAPI 3.0 Schema patterns for each corresponding models.

New-type.

    openapi: 3.0.1
    info:
        title: Swagger Petstore
        version: 1.2.3
    paths: {}
    components:
        schemas:
            Order: { type: string }

Sum-type.

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
            Cat: { type: string }
            Dog: { type: string }

Product-type.

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

See `src/scan/openapi3/test.rs` module for complete examples.





Design Choices
--------------
- Accepts a very narrow strict subset of OpenAPI 3.0 schema.
- Prouces working Rust code.
    - Use Rust compiler for static schema validation.
    - Use compiled Rust program for dynamic data instance validation.
- Only these features will be supported.
    - New-type.
    - Sum-type. (both name-based and type-based variants)
    - Product-type.
    - Primitive-type. (`bool`, `i32`, `i64`, `f32`, `f64`, `String`)
    - NO inline structures. (not even tuples)
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
Copyright(c) 2022, Eonil. All rights reserved.