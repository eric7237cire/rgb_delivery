import {Component, OnChanges, OnInit, SimpleChanges} from '@angular/core';
import * as _ from "lodash";
import {
  CellData,
  ChoiceOverride,
  Color,
  TileEnum,
  TileEnum_type,
  TileRoad,
  Universe,
  UniverseData,
  Van
} from "../../../rgb-solver/pkg";
import {GridStorageService} from "./grid-storage.service";
import {DomSanitizer, SafeUrl} from "@angular/platform-browser";


interface DirectionMarker {
  text: string,
  x_offset: number,
  y_offset: number,
  annotation_x_offset: number,
  annotation_y_offset: number,
  mask: number,
  dir_index: number
}

enum DIRECTION_INDEX {
    NORTH=0,
    EAST=1,
    SOUTH=2,
    WEST=3
}


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
      x_offset: this.GRID_SIZE /2 ,
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
      x_offset: this.GRID_SIZE /2 ,
      y_offset: this.GRID_SIZE * 0.8,
      //south east corner
      annotation_x_offset: this.GRID_SIZE /2 + 5,
      annotation_y_offset: this.GRID_SIZE * 0.8,
      mask: 4,
      dir_index: 2
    },
    //east
    {
      text: "-",
      x_offset: this.GRID_SIZE * 0.8 ,
      y_offset: this.GRID_SIZE * 0.5,
      //north/east corner
      annotation_x_offset: this.GRID_SIZE * 0.8 -10 ,
      annotation_y_offset: this.GRID_SIZE * 0.25,
      mask: 2,
      dir_index: 1
    },
    //west
    {
      text: "-",
      x_offset: this.GRID_SIZE * 0.25 ,
      y_offset: this.GRID_SIZE * 0.5,
      //south west corner
      annotation_x_offset: 2,
      annotation_y_offset: this.GRID_SIZE * 0.8,
      mask: 8,
      dir_index: 3
    },

    ];

  num_cols: number = null;
  num_rows: number = null;

  colors: Array<Color> = [];

  readonly tiles: Array<TileEnum_type> = ["TileRoad", "Empty", "TileWarehouse"];

  universe: Universe = null;

  universeData: UniverseData = null;

  selectedColor = this.colors[0];
  selectedThing: Thing = "Van";
  readonly THING_LIST: Array<Thing> = ["Van", "Block", "Clear"];

  selectedTile: TileEnum_type = this.tiles[0];

  jsonSaveAs: SafeUrl;

  wasm: typeof import('../../../rgb-solver/pkg');

  numCalcSteps=29;

  mouseMoveRow = 0;
  mouseMoveCol = 0;

  constructor(private gridStorageService: GridStorageService, private sanitizer: DomSanitizer) {
  }

  setGridSquare(cellData: CellData) {
    this.universe.set_square(cellData);


  }

  handleFileSelect(evt) {
    const files = evt.target.files; // FileList object

    // files is a FileList of File objects. List some properties.
    var output = [];
    let file: File = files[0];


    const reader = new FileReader();

    reader.onload = () => {
      let fileTextData: string = reader.result as string;

      let data: UniverseData = JSON.parse(fileTextData);

      this.num_rows = data.height;
      this.num_cols = data.width;

      this.gridStorageService.storeGrid(data);

      this.loadGridJsonData(data);
    };


    reader.readAsText(file);


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

    this.loadGridJsonData(savedData);
  }

  loadGridJsonData(jsonData: UniverseData) {

    console.log("Loading json data", jsonData);

    this.universe = this.wasm.Universe.new(this.num_rows, this.num_cols);

    console.log("Universe initial data", this.universe.get_data());

    //Giving web assembly the loaded data
    if (!_.isNil(jsonData) && _.isArray(jsonData.cells)) {
      jsonData.cells.forEach((cellData: CellData) => {


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

    this.initCalculations();

  }

  handleWasmLoaded(mymod: typeof import('rgb-solver')) {
    console.log("All modules loaded");
    this.wasm = mymod;

    this.updateDim();


    let overRideList: Array<ChoiceOverride> = [

     /* {
        row_index: 6,
        col_index: 7,
        van_index: 1,
        direction_index: DIRECTION_INDEX.WEST
      },
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

    this.universe.set_overrides(overRideList);

    this.nextCalculateStep(this.numCalcSteps);

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

  onThingClick(thing) {
    this.selectedThing = thing;
  }

  getCssForColor(c: Color) {
    if (_.isNil(c)) {
      return "";
    }
    return `rgb(${c.red}, ${c.green}, ${c.blue})`;
  }

  handleMouseMove(moveEvent: MouseEvent) {
    const rect = (<any>moveEvent.target).getBoundingClientRect();

    const x = moveEvent.clientX - rect.left; //x position within the element.
    const y = moveEvent.clientY - rect.top;  //y position within the element.

    const col_index = _.floor(x / this.GRID_SIZE);
    const row_index = _.floor(y / this.GRID_SIZE);

    this.mouseMoveRow = row_index;
    this.mouseMoveCol = col_index;
  }

  handleGridClick(clickEvent: MouseEvent, isRightClick: boolean): boolean {
    console.log(clickEvent);
    console.log(clickEvent.target);

    const rect = (<any>clickEvent.target).getBoundingClientRect();

    const x = clickEvent.clientX - rect.left; //x position within the element.
    const y = clickEvent.clientY - rect.top;  //y position within the element.

    const col_index = _.floor(x / this.GRID_SIZE);
    const row_index = _.floor(y / this.GRID_SIZE);

    console.log(`Clicked on row ${row_index}, col ${col_index}.  Right click? ${isRightClick}`);

    if (isRightClick) {
      let cellIndex = row_index * this.num_cols + col_index;
      let tile : TileEnum = this.universeData.cells[cellIndex].tile;

      switch (tile.type) {
        case "TileRoad": {

          switch (this.selectedThing) {
            case "Van":
              tile.van = {boxes: [null, null, null], color: this.selectedColor, is_done: false};
              break;
            case "Block":
              tile.block = this.selectedColor;
              break;
            case "Clear":
              tile.block = null;
              tile.van = null;
              break;
          }

          this.setGridSquare({row_index, col_index, tile});
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
          this.setGridSquare({row_index, col_index, "tile": {type: this.selectedTile}});
          break;
        case "TileRoad":
          this.setGridSquare({
            row_index, col_index, "tile": {
              type: this.selectedTile,
              used_mask: 0,
              used_tick: null,
              used_van_index: null
            }
          });
          break;
        case "TileWarehouse":
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

    var theJSON = JSON.stringify(this.universeData, null, 2);
    this.jsonSaveAs = this.sanitizer.bypassSecurityTrustUrl(
      "data:text/json;charset=UTF-8," + encodeURIComponent(theJSON));


    return false;
    //console.log(clickEvent.target.getBoundingClientRect());
  }

  handleClickSvgText() {
    return true;
  }


  getCellColor(cell: CellData): string {
    switch (cell.tile.type) {
      case "TileWarehouse":
        let w = cell.tile;
        return this.getCssForColor(w.color);

      case "Empty":
        return "rgb(100,100,100)";

      default:
        return "rgb(200,200,200)";
    }

  }

  isDirectionMarkerVisible(cell: CellData, dm: DirectionMarker) : boolean {
    if (cell.tile.type != "TileRoad") {
      return false;
    }

    return ( (cell.tile.used_mask & dm.mask) > 0 );
  }

  readonly DEFAULT_DM_COLOR="rgb(200, 200, 200)";
  getCssColorForDirectionMarker(cell: CellData, dm: DirectionMarker) : string {
    //color of the van


    if (cell.tile.type != "TileRoad") {
      return this.DEFAULT_DM_COLOR;
    }
    let road: TileRoad = cell.tile;
    let van_index = road.used_van_index[dm.dir_index];

    if (_.isNil(van_index)) {
      return this.DEFAULT_DM_COLOR;
    }

    return this.getCssForColor(this.universeData.vans[van_index].color);
  }

  getDirectionMarkerAnnotation(cell: CellData, dm: DirectionMarker) : string {
    if (cell.tile.type != "TileRoad") {
      return null;
    }
    let road: TileRoad = cell.tile;
    let van_index = road.used_van_index[dm.dir_index];

    if (_.isNil(van_index)) {
      return null;
    }

    return `${road.used_tick[dm.dir_index]}`;
    //return `${van_index}: ${road.used_tick[dm.dir_index]}`;
  }

  initCalculations() {
    this.universe.init_calculate();

    let data: UniverseData = this.universe.get_data();

    this.universeData = data;
  }

  nextCalculateStep(numStepsParam: number) {
    let numSteps = _.toNumber(numStepsParam);

    for (let i = 0; i < numSteps; i+=1) {
      let data: UniverseData = this.universe.next_calculate();

      this.universeData = data;
    }

    //strip out empty cells
    if (!_.isNil(this.universeData)) {
      this.universeData.cells = this.universeData.cells.filter(
        cell => cell.tile.type != "Empty"
      )
    };

    console.log("After calculations", this.universeData);
  }

}
