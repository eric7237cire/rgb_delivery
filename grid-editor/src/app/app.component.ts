import {Component, OnChanges, OnInit, SimpleChanges} from '@angular/core';
import * as _ from "lodash";
import {CellData, Color, TileType, Universe, UniverseData} from "../../../rgb-solver/pkg";
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

  num_cols: number = null;
  num_rows: number = null;

  colors: Array<Color> = [];

  readonly tiles: Array<TileType> = ["Road", "Empty", "Warehouse"];

  universe: Universe = null;

  universeData: UniverseData = null;

  selectedColor = this.colors[0];
  selectedTile: TileType = this.tiles[0];


  wasm: typeof import('../../../rgb-solver/pkg');

  constructor(private gridStorageService: GridStorageService) {
  }

  setGridSquare(cellData: CellData) {
    this.universe.set_square(cellData);
  }

  updateDim() {
    //if (!_.isNil(changes.num_cols) || !_.isNil(changes.num_rows) ) {

    let savedData = this.gridStorageService.loadGrid();

    this.num_rows = _.toNumber(this.num_rows);
    this.num_cols = _.toNumber(this.num_cols);

    if (!_.isNil(savedData) && _.isArray(savedData.cells) && this.num_cols * this.num_rows == 0) {
      this.num_rows = savedData.height;
      this.num_cols = savedData.width;
    }



    this.universe = this.wasm.Universe.new(this.num_rows, this.num_cols);

    console.log("Universe initial data", this.universe.get_data());

    //Giving web assembly the loaded data
    if (!_.isNil(savedData) && _.isArray(savedData.cells)) {
      savedData.cells.forEach((cellData: CellData) => {


          this.setGridSquare(cellData);


      });
    }

    console.log("My universe", this.universe.render());

    this.universeData = this.universe.get_data();
    this.colors = this.wasm.get_colors();


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

  handleGridClick(clickEvent: MouseEvent, clearSquare: boolean): boolean {
    console.log(clickEvent);
    console.log(clickEvent.target);

    const rect = (<any>clickEvent.target).getBoundingClientRect();

    const x = clickEvent.clientX - rect.left; //x position within the element.
    const y = clickEvent.clientY - rect.top;  //y position within the element.

    const col_index = _.floor(x / this.GRID_SIZE);
    const row_index = _.floor(y / this.GRID_SIZE);

    console.log(`Clicked on row ${row_index}, col ${col_index}.  Clear? ${clearSquare}`);

    if (clearSquare) {
      this.setGridSquare({row_index, col_index, tile: {type: "Empty"}});
    } else {
      switch (this.selectedTile) {
        case "Empty":
          this.setGridSquare({row_index, col_index, "tile": {type: this.selectedTile}});
          break;
        case "Road":
          this.setGridSquare({
            row_index, col_index, "tile": {
              type: this.selectedTile,
              used_mask: 0
            }
          });
          break;
        case "Warehouse":
          this.setGridSquare({
            row_index, col_index, "tile": {
              type: this.selectedTile,
              color: this.selectedColor,
              is_filled: false
            }
          });
          break;
      }
      //this.universe.set_square(rowIndex, colIndex, this.selectedColor, this.selectedTile);
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

  getCellColor(cell: CellData) : string {
    switch (cell.tile.type) {
      case "Warehouse":
        let w = cell.tile;
        return `rgb(${w.color.red},${w.color.green},${w.color.blue}`;

      default: return "rgb(200,200,200)";
    }

  }
}
