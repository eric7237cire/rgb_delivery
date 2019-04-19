import {Component, OnInit} from '@angular/core';
import * as _ from "lodash";
import {
  CellData,
  ChoiceOverride,
  Color,
  ColorIndex,
  GridState,
  TileEnum,
  TileEnum_type,
  TileRoad
} from "../../../rgb-solver/pkg";
import {GridStorageService} from "./grid-storage.service";
import {DomSanitizer, SafeUrl} from "@angular/platform-browser";
import {GridStateService} from "./grid-state.service";


interface DirectionMarker {
  text: string;
  x_offset: number;
  y_offset: number;
  annotation_x_offset: number;
  annotation_y_offset: number;
  mask: number;
  dir_index: number;
}

enum DIRECTION_INDEX {
  NORTH = 0,
  EAST = 1,
  SOUTH = 2,
  WEST = 3
}

const DEFAULT_DM_COLOR = "rgb(200, 200, 200)";

export type WASM_TYPE = typeof import ('../../../rgb-solver/pkg');

type Thing = "Van" | "Block" | "Clear";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.styl']
})
export class AppComponent implements OnInit {
  title = 'grid-editor';
  readonly GRID_SIZE = 40;

  readonly DIRECTION_MARKERS: Array<DirectionMarker> = [
    //north
    {
      text: "|",
      x_offset: this.GRID_SIZE / 2,
      y_offset: this.GRID_SIZE * 0.25,
      //north west corner
      annotation_x_offset: 2,
      annotation_y_offset: 10,
      mask: 1,
      dir_index: 0
    },
    //south
    {
      text: "|",
      x_offset: this.GRID_SIZE / 2,
      y_offset: this.GRID_SIZE * 0.8,
      //south east corner
      annotation_x_offset: this.GRID_SIZE / 2 + 5,
      annotation_y_offset: this.GRID_SIZE * 0.8,
      mask: 4,
      dir_index: 2
    },
    //east
    {
      text: "-",
      x_offset: this.GRID_SIZE * 0.8,
      y_offset: this.GRID_SIZE * 0.5,
      //north/east corner
      annotation_x_offset: this.GRID_SIZE * 0.8 - 10,
      annotation_y_offset: this.GRID_SIZE * 0.25,
      mask: 2,
      dir_index: 1
    },
    //west
    {
      text: "-",
      x_offset: this.GRID_SIZE * 0.25,
      y_offset: this.GRID_SIZE * 0.5,
      //south west corner
      annotation_x_offset: 2,
      annotation_y_offset: this.GRID_SIZE * 0.8,
      mask: 8,
      dir_index: 3
    },

  ];

  numCols: number = 3;
  numRows: number = 3;

  colors: Array<Color> = [];

  readonly tiles: Array<TileEnum_type> = ["TileRoad", "Empty", "TileWarehouse"];

  selectedColor = this.colors[0];
  selectedThing: Thing = "Van";
  readonly THING_LIST: Array<Thing> = ["Van", "Block", "Clear"];

  selectedTile: TileEnum_type = this.tiles[0];

  jsonSaveAs: SafeUrl;

  wasm: typeof import ('../../../rgb-solver/pkg');

  numCalcSteps = 300;

  mouseMoveRow = 0;
  mouseMoveCol = 0;

  constructor(
    private gridStorageService: GridStorageService,
    private sanitizer: DomSanitizer,
    public gridStateService: GridStateService
  ) {
  }

  handleWasmLoaded(newWasm: WASM_TYPE) {
    this.wasm = newWasm;

    this.colors = this.wasm.get_colors();


    if (!this.selectedColor) {
      this.selectedColor = this.colors[1];
    }

    if (!this.selectedTile) {
      this.selectedTile = this.tiles[0];
    }
    console.log("Color", this.colors);
    console.log("Tiles", this.tiles);

    console.log("All modules loaded");


    //see if we have one from local storage
    const savedData = this.gridStorageService.loadGrid();

    if (!_.isNil(savedData)) {
      this.gridStateService.loadGridState(savedData, this.wasm);
      this.numRows = savedData.height;
      this.numCols = savedData.width;

      this.initCalculations();
    }
    //this.nextCalculateStep(this.numCalcSteps);

    this.gridStateService.gridState$.subscribe(newVal => {
      this.handleGridStateChanged();
    });

  }

  handleGridStateChanged() {
    console.log("Data is now", this.gridStateService.gridState);

    let nonEmptyCells: Array<CellData> = [];
    //strip out empty cells
    if (!_.isNil(this.gridStateService.gridState)) {
      nonEmptyCells = this.gridStateService.gridState.cells.filter(
        cell => cell.tile.type !== "Empty"
      );
    }

    console.log("Non empty cells", nonEmptyCells);

    //Only continue processing on initial tick/load
    if (this.gridStateService.gridState.tick > 0) {
      return;
    }

    const overRideList: Array<ChoiceOverride> = [


      /*
       {
         row_index: 6,
         col_index: 5,
         van_index: 1,
         direction_index: DIRECTION_INDEX.NORTH
       },
       {
         row_index: 4,
         col_index: 5,
         van_index: 1,
         direction_index: DIRECTION_INDEX.NORTH
       },
       {
         row_index: 2,
         col_index: 5,
         van_index: 1,
         direction_index: DIRECTION_INDEX.WEST
       },
       {
         row_index: 2,
         col_index: 3,
         van_index: 1,
         direction_index: DIRECTION_INDEX.NORTH
       },


       {
         row_index: 8,
         col_index: 5,
         van_index: 0,
         direction_index: DIRECTION_INDEX.NORTH
       },
       {
         row_index: 6,
         col_index: 3,
         van_index: 0,
         direction_index: DIRECTION_INDEX.SOUTH,
         tick: 8
       },*/
    ];

    this.gridStateService.universe.set_overrides(overRideList);

    this.gridStorageService.storeGrid(this.gridStateService.gridState);

    const theJSON = JSON.stringify(this.gridStateService.gridState, null, 2);
    this.jsonSaveAs = this.sanitizer.bypassSecurityTrustUrl(
      "data:text/json;charset=UTF-8," + encodeURIComponent(theJSON));


  }


  handleFileSelect(evt) {
    const files = evt.target.files; // FileList object

    // files is a FileList of File objects. List some properties.
    const output = [];
    const file: File = files[0];


    const reader = new FileReader();

    reader.onload = () => {
      const fileTextData: string = reader.result as string;

      const data: GridState = JSON.parse(fileTextData);

      this.numRows = data.height;
      this.numCols = data.width;

      this.gridStorageService.storeGrid(data);

      this.loadGridJsonData(data);
    };


    reader.readAsText(file);


  }

  /**
   * From UI
   */
  handleWidthHeightChange() {


    this.numRows = _.toNumber(this.numRows);
    this.numCols = _.toNumber(this.numCols);

    this.gridStateService.gridState.width = this.numCols;
    this.gridStateService.gridState.height = this.numRows;

    this.loadGridJsonData(this.gridStateService.gridState);
  }

  loadGridJsonData(jsonData: GridState) {


    this.gridStateService.loadGridState(jsonData, this.wasm);


    this.initCalculations();

  }


  async load() {
    this.handleWasmLoaded(await import('rgb-solver'));
  }

  ngOnInit(): void {

    console.log("ng on init");

    //RustRGBProject/pkg works but not in PyCharm


    this.load().then(() => {
      console.log("Load done");
    });

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

  onThingClick(thing) {
    this.selectedThing = thing;
  }

  getCssForColor(c: Color) {
    if (_.isNil(c)) {
      return "";
    }
    return `rgb(${c.red}, ${c.green}, ${c.blue})`;
  }

  getCssForColorIndex(ci: ColorIndex) {
    return this.getCssForColor(this.colors[ci]);
  }

  handleMouseMove(moveEvent: MouseEvent) {
    const rect = (moveEvent.target as any).getBoundingClientRect();

    const x = moveEvent.clientX - rect.left; //x position within the element.
    const y = moveEvent.clientY - rect.top;  //y position within the element.

    const colIndex = _.floor(x / this.GRID_SIZE);
    const rowIndex = _.floor(y / this.GRID_SIZE);

    this.mouseMoveRow = rowIndex;
    this.mouseMoveCol = colIndex;
  }

  handleGridClick(clickEvent: MouseEvent, isRightClick: boolean): boolean {
    console.log(clickEvent);
    console.log(clickEvent.target);

    const rect = (clickEvent.target as any).getBoundingClientRect();

    const x = clickEvent.clientX - rect.left; //x position within the element.
    const y = clickEvent.clientY - rect.top;  //y position within the element.

    const colIndex = _.floor(x / this.GRID_SIZE);
    const rowIndex = _.floor(y / this.GRID_SIZE);

    console.log(`Clicked on row ${rowIndex}, col ${colIndex}.  Right click? ${isRightClick}`);

    if (isRightClick) {
      const cellIndex = rowIndex * this.numCols + colIndex;
      const tile: TileEnum = this.gridStateService.gridState.cells[cellIndex].tile;

      switch (tile.type) {
        case "TileRoad": {

          switch (this.selectedThing) {
            case "Van":
              tile.van = {boxes: [null, null, null], color: this.selectedColor.color_index, is_done: false};
              break;
            case "Block":
              tile.block = this.selectedColor.color_index;
              break;
            case "Clear":
              delete tile.block;
              delete tile.van;
              break;
          }

          this.gridStateService.setGridSquare({row_index: rowIndex, col_index: colIndex, tile});
          break;
        }
        default: {
          console.log("Not a road");
          break;
        }
      }

    } else {
      switch (this.selectedTile) {
        case "Empty":
          this.gridStateService.setGridSquare({
            row_index: rowIndex,
            col_index: colIndex,
            tile: {type: this.selectedTile}
          });
          break;
        case "TileRoad":
          this.gridStateService.setGridSquare({
            row_index: rowIndex, col_index: colIndex, tile: {
              type: this.selectedTile,
              used_mask: 0,
            }
          });
          break;
        case "TileWarehouse":
          this.gridStateService.setGridSquare({
            row_index: rowIndex, col_index: colIndex, tile: {
              type: this.selectedTile,
              color: this.selectedColor.color_index,
              is_filled: false
            }
          });
          break;
      }
      //this.universe.set_square(rowIndex, colIndex, this.selectedColor, this.selectedTile);
    }


    return false;
    //console.log(clickEvent.target.getBoundingClientRect());
  }

  handleClickSvgText() {
    return true;
  }


  getCellColor(cell: CellData): string {
    switch (cell.tile.type) {
      case "TileWarehouse":
        const w = cell.tile;
        return this.getCssForColorIndex(w.color);

      case "Empty":
        return "rgb(100,100,100)";

      default:
        return "rgb(200,200,200)";
    }

  }

  isDirectionMarkerVisible(cell: CellData, dm: DirectionMarker): boolean {
    if (cell.tile.type !== "TileRoad") {
      return false;
    }

    return ((cell.tile.used_mask & dm.mask) > 0);
  }


  getCssColorForDirectionMarker(cell: CellData, dm: DirectionMarker): string | null {
    //color of the van


    if (cell.tile.type !== "TileRoad") {
      return DEFAULT_DM_COLOR;
    }
    const road: TileRoad = cell.tile;

    if (_.isNil(road.used_van_index)) {
      return null;
    }

    const vanIndex = road.used_van_index[dm.dir_index];

    if (_.isNil(vanIndex)) {
      return DEFAULT_DM_COLOR;
    }

    return this.getCssForColorIndex(this.gridStateService.gridState.vans[vanIndex].color);
  }

  getDirectionMarkerAnnotation(cell: CellData, dm: DirectionMarker): string | null {
    if (cell.tile.type !== "TileRoad") {
      return null;
    }
    const road: TileRoad = cell.tile;

    if (_.isNil(road.used_van_index)) {
      return null;
    }

    const vanIndex = road.used_van_index[dm.dir_index];

    if (_.isNil(vanIndex) || _.isNil(road.used_tick)) {
      return null;
    }

    return `${road.used_tick[dm.dir_index]}`;
    //return `${van_index}: ${road.used_tick[dm.dir_index]}`;
  }

  initCalculations() {
    this.gridStateService.universe.init_calculate();
    this.gridStateService.reloadGridData();
  }

  nextCalculateStep(numStepsParam: number) {
    const numSteps = _.toNumber(numStepsParam);

    this.gridStateService.gridState$.next(this.gridStateService.universe.next_batch_calculate(numSteps));


  }

}
