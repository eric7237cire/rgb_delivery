[![Build Status](https://travis-ci.org/eric7237cire/poker.svg?branch=master)](https://travis-ci.org/eric7237cire/poker)

# ![Rust](http://rust-lang.org/logos/rust-logo-32x32.png) A grid solver for [RGB Express](http://rgbexpress.com/) ![Rust](http://rust-lang.org/logos/rust-logo-32x32.png) 

## Components

### grid-editor

Angular 7 front end, grid editor.  

![Screenshot](./readme_images/grid_editor.png)

Language: TypeScript

### web_worker_build

Packages the WASM using a stand alone webpack as a Web worker.  Too much of a PITA to get angular cli
to play nicely.  Copies the output directly to angulars assets.

Language: TypeScript

### rgb-solver

WASM Source.  Unit tests run in x86 (or linux on travis)

Language: Rust

### wasm-typescript-definition

Code based on a [public repo](https://github.com/tcr/wasm-typescript-definition) of same name.

Modified to produce better *.d.ts types for typescript.  Most useful is generating union types which match nicely with Rusts variant enums.

TypeScript has nice support for [discriminated unions](https://basarat.gitbooks.io/typescript/docs/types/discriminated-unions.html) which let you do a checked switch on the attribute type.

Example:

Typing generated:

```typescript
export type Road = {
    ...
};

export type Bridge = {
    ...
};

//derive struct
export type Warehouse = {
    ...
};

export type TileEnum = TileRoad | TileWarehouse | TileBridge | Empty

export type TileEnum = TileRoad | TileWarehouse | TileBridge | Empty
export type TileRoad = {type: "TileRoad"} & Road
export type TileWarehouse = {type: "TileWarehouse"} & Warehouse
export type TileBridge = {type: "TileBridge"} & Bridge
export type Empty = {type: "Empty" }
export type TileEnum_type = "TileRoad" | "TileWarehouse" | "TileBridge" | "Empty"
```

Rust:

```rust
#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum TileEnum {
    TileRoad(Road),
    TileWarehouse (Warehouse),
    TileBridge(Bridge),
    Empty
}
```
