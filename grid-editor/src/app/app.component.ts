import {Component, OnChanges, OnInit, SimpleChanges} from '@angular/core';
import * as _ from "lodash";
import {Color, Tile} from "../../../rgb-solver/pkg";
/*
import loadWasm from '../../../rgb-solver/src/lib.rs';
console.log('I am alive!!!');
loadWasm().then(result => {
  const {add, subtract, multiply} = result.instance.exports;
  console.log('4 + 2 = ', add(4, 2));
  console.log('4 - 2 = ', subtract(4, 2));
  console.log('4 * 2 = ', multiply(4, 2));
});
*/

/*

*/

/*
const wasm = import("../../../rgb-solver/pkg/rgb_solver");

wasm.then(module => {
  // won't typecheck if yourlib does not expose the run function
  module.greet();
});*/


class TileType {
  key: string;
  label: string;

  constructor(_k, _l) {
    this.key = _k;
    this.label = _l;
  }
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.styl']
})
export class AppComponent implements OnInit, OnChanges {
  title = 'grid-editor';
  grid_size = 40;

  num_cols: number = 10;
  num_rows: number = 10;

  colors: Array<Color> = [];

  tiles: Array<Tile> = [];

  selectedColor = this.colors[0];
  selectedTile = this.tiles[0];


  wasm: typeof import('../../../rgb-solver/pkg');

  ngOnChanges(changes: SimpleChanges): void {

    console.log("ng on changes");


  }

  updateDim() {
    //if (!_.isNil(changes.num_cols) || !_.isNil(changes.num_rows) ) {

    this.num_rows = _.toNumber(this.num_rows);
    this.num_cols = _.toNumber(this.num_cols);

    let u = this.wasm.Universe.new(this.num_rows, this.num_cols);

    console.log("My universe", u.render());

    this.colors = u.get_colors();
    this.tiles = u.get_tiles();

    if (!this.selectedColor) {
      this.selectedColor = this.colors[1];
    }

    if (!this.selectedTile) {
      this.selectedTile = this.tiles[0];
    }
    console.log("Color", this.colors);
    console.log("Tiles", this.tiles);


  }

  handleWasmLoaded(mymod: typeof import('rgb-solver')) {
    console.log("All modules loaded");
    this.wasm = mymod;
    //mymod.greet();
    this.updateDim();
  }

  async load() {
    this.handleWasmLoaded(await import('rgb-solver'));
  }

  ngOnInit(): void {

    console.log("ng on init");

    //RustRGBProject/pkg works but not in PyCharm

    this.load();
    /*
    syntax when using wasm plugin
    import("../../../rgb-solver/pkg").then(module => {

      this.wasm = module;

      this.updateDim();
    });*/
  }

  onTileClick(t) {
    this.selectedTile = t;
  }

  onColorClick(c) {
    this.selectedColor = c;
  }

  getCssForColor(c: Color) {
    if (_.isNil(c)) {
      return "";
    }
    return `rgb(${c.red}, ${c.green}, ${c.blue})`;
  }

}
