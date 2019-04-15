import {Component, OnChanges, OnInit, SimpleChanges} from '@angular/core';
import * as _ from "lodash";
import {CellData, Color, Tile, Universe, UniverseData} from "../../../rgb-solver/pkg";
import {GridStorageService} from "./grid-storage.service";
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



@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.styl']
})
export class AppComponent implements OnInit {
  title = 'grid-editor';
  readonly GRID_SIZE = 40;

  num_cols: number = 3;
  num_rows: number = 2;

  colors: Array<Color> = [];

  tiles: Array<string> = ["Road", "Empty", "Warehouse"];

  universe: Universe = null;

  universeData: UniverseData = null;

  selectedColor = this.colors[0];
  selectedTile = this.tiles[0];


  wasm: typeof import('../../../rgb-solver/pkg');

  constructor(private gridStorageService: GridStorageService) {}

  setGridSquare(rowIndex: number, colIndex:number, cellData: CellData) {
    this.universe.set_square(rowIndex,colIndex, cellData);
  }

  updateDim() {
    //if (!_.isNil(changes.num_cols) || !_.isNil(changes.num_rows) ) {

    let savedData = this.gridStorageService.loadGrid();

    if (!_.isNil(savedData) && _.isArray(savedData.cells)) {
      this.num_rows = savedData.height;
      this.num_cols = savedData.width;
    }

    this.num_rows = _.toNumber(this.num_rows);
    this.num_cols = _.toNumber(this.num_cols);

    this.universe = this.wasm.Universe.new(this.num_rows, this.num_cols);


    //Giving web assembly the loaded data
    if (!_.isNil(savedData) && _.isArray(savedData.cells)) {
      savedData.cells.forEach( (cellData: CellData, index) => {
        let row = _.floor( index / this.num_cols);
        let col = index % this.num_cols;

        if (_.isNil(cellData)) {
          this.setGridSquare(row,col, null);
        } else {
          this.universe.set_square(row, col, cellData);
        }

      });
    }

    console.log("My universe", this.universe.render());

    this.universeData = this.universe.get_data();
    this.colors = this.wasm.get_colors();
    this.tiles = this.wasm.get_tiles();

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

  handleGridClick(clickEvent: MouseEvent, clearSquare: boolean) : boolean {
    console.log(clickEvent);
    console.log(clickEvent.target);

    const rect = ( <any>clickEvent.target).getBoundingClientRect();

    const x = clickEvent.clientX - rect.left; //x position within the element.
    const y = clickEvent.clientY - rect.top;  //y position within the element.

    const colIndex = _.floor(x / this.GRID_SIZE);
    const rowIndex = _.floor(y / this.GRID_SIZE);

    console.log(`Clicked on row ${rowIndex}, col ${colIndex}.  Clear? ${clearSquare}`);

    if (clearSquare) {
      this.setGridSquare(rowIndex, colIndex, null);
    } else {
      this.universe.set_square(rowIndex, colIndex, this.selectedColor, this.selectedTile);
    }

    this.universeData = this.universe.get_data();

    console.log("Data is now", this.universeData);

    this.gridStorageService.storeGrid(this.universeData);

    return false;
    //console.log(clickEvent.target.getBoundingClientRect());
  }

  handleClickSvgText() {
    return true;
  }
}
