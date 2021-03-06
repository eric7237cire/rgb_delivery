[![Build Status](https://travis-ci.org/eric7237cire/poker.svg?branch=master)](https://travis-ci.org/eric7237cire/poker)

# ![Rust](http://rust-lang.org/logos/rust-logo-32x32.png) 🦀 🕸️ A grid solver for [RGB Express](http://rgbexpress.com/) ![Rust](http://rust-lang.org/logos/rust-logo-32x32.png) 

[RGB Express](http://rgbexpress.com/) is an excellent strategy Android game.  It helps to download & play before looking at the solver.

Basically you have to get each van (V) pick up a block (B) and return it to the target warehouse (T).   The van and warehouse must be the same color.
Each van can hold 3 blocks, the 'poppers', if active, will pop the top block off the Van.  Poppers are activated before starting the level.  


Component/tech flow:

WASM => Web Worker <=> Angular 7

Thus Angular has no knowledge of the WebAssembly, using only WebWorker messages, but it does use the typings generated.  Thus if a new attribute/etc. is 
added to the public classes/interfaces (exposed by Serde), the typescript transpilation will show errors.


## The Algorithm

Just a brute force search with some pruning done with connected components calculated 
with the Union Find / Disjoint Set datastructure.

## Components

### grid-editor

Angular 7 front end, grid editor & runs the search.

![Screenshot](./readme_images/grid_editor_42.gif)

Language: TypeScript

### web_worker

Code based on a [public repo](https://github.com/rustwasm/rust-wasm-worker-template).

Packages the WASM using a stand alone webpack as a Web worker.  Too much of a PITA to get angular cli
to play nicely.  

Build as a local only npm module, exposing a browser asset and the typings.

Language: TypeScript

To setup, see the [Travis Configuration](.travis.yml) 



### rgb-solver

WASM Source.  Unit tests run in x86 (or linux on travis)

Language: Rust

To setup, see the [The Watch Bat File](/devops/build-wasm.bat) 



### wasm-typescript-definition

Code based on a [public repo](https://github.com/tcr/wasm-typescript-definition) of same name.

Modified to produce better *.d.ts types for typescript.  Most useful is generating union types which match nicely with Rusts variant enums.

TypeScript has nice support for [discriminated unions](https://basarat.gitbooks.io/typescript/docs/types/discriminated-unions.html) which let you do a checked switch on the attribute type.

Built automatically when rgb-solver is built.

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


# Deploying to [git hub pages](https://eric7237cire.github.io/rgb_delivery/)

Done by travis, see the [Travis Configuration](.travis.yml) 
