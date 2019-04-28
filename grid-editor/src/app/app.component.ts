import {Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import * as _ from "lodash";
import {
  Button,
  CellData,
  ChoiceOverride,
  Color,
  ColorIndex,
  GridState,
  TileEnum,
  TileEnum_type,
  TileRoad
} from "web_worker";
import {GridStorageService} from "./grid-storage.service";
import {DomSanitizer, SafeUrl} from "@angular/platform-browser";
import { Subject} from "rxjs";
import {mergeMap, takeUntil, throttleTime} from "rxjs/operators";

import {
  RequestInitCalculations,
  RequestLoadGridState,
  RequestRunCalculateSteps,
  RequestSetGridSquare,
  RequestSetOverrideList,
  RequestTypes,
  RequestWasmLoad,
  ResponseTypes,
  ResponseWasmLoaded,
  WasmWebWorkerResponse
} from "web_worker";
import {LogService} from "./log.service";



interface DirectionMarker {
  text: string;
  x_offset: number;
  y_offset: number;
  annotation_x_offset: number;
  annotation_y_offset: number;
  mask: number;
  dir_index: DIRECTION_INDEX;
}

enum DIRECTION_INDEX {
  NORTH = 0,
  EAST = 1,
  SOUTH = 2,
  WEST = 3
}

const LOCAL_STORAGE_KEY_NUM_STEPS = "numSteps";

const DEFAULT_DM_COLOR = "rgb(200, 200, 200)";

type Thing = "Van" | "Block" | "Button" | "Clear" | "Popper";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./style/app.component.styl']
})
export class AppComponent implements OnInit {
  title = 'grid-editor';
  readonly GRID_SIZE = 40;

  @ViewChild('fileInput')
  private fileInput: ElementRef;

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
      dir_index: DIRECTION_INDEX.NORTH
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
      dir_index: DIRECTION_INDEX.SOUTH
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
      dir_index: DIRECTION_INDEX.EAST
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
      dir_index: DIRECTION_INDEX.WEST
    },

  ];

  numCols: number = 3;
  numRows: number = 3;
  gridState: GridState;

  //when changing dimensions, reapply tiles
  tempTiles: Array<CellData> = [];

  colors: Array<Color> = [];

  readonly tiles: Array<TileEnum_type> = ["TileRoad", "Empty", "TileWarehouse", "TileBridge"];

  selectedColor = this.colors[0];
  selectedThing: Thing = "Van";
  readonly THING_LIST: Array<Thing> = ["Van", "Block", "Button", "Clear", "Popper"];

  selectedTile: TileEnum_type = this.tiles[0];

  selectedIsOpenOrUp: boolean = true;

  isDirectionOn: Array<boolean> = [true, true, true, true];
  directions: Array<string> = ["North", "East",
    "South", "West"
  ];

  jsonSaveAs: SafeUrl;

  worker: Worker;

  cells: Array<CellData> = [];

  numCalcSteps = 100000;

  mouseMoveRow = "00";
  mouseMoveCol = "00";
  mouseMoveIndex = "00";

  progressMessage: string = "No Progress Info";

  readonly gridMouseMove$ = new Subject<MouseEvent>();
  readonly gridMouseDown$ = new Subject<MouseEvent>();
  readonly gridMouseUp$ = new Subject<MouseEvent>();

  constructor(
    private gridStorageService: GridStorageService,
    private sanitizer: DomSanitizer,
    private log: LogService
  ) {
  }

  handleWasmLoaded(wasmLoadedMessage: ResponseWasmLoaded) {


    this.colors = wasmLoadedMessage.colors;


    if (!this.selectedColor) {
      this.selectedColor = this.colors[1];
    }

    if (!this.selectedTile) {
      this.selectedTile = this.tiles[0];
    }
    this.log.debug("Color", this.colors);
    this.log.debug("Tiles", this.tiles);

    this.log.info("All modules loaded");


    //see if we have one from local storage
    const savedData = this.gridStorageService.loadGrid();
    this.loadJsonFromClient(savedData);
  }

  sendOverrideList() {
    const overRideList: Array<ChoiceOverride> = [


/*
      {
         row_index: 0,
         col_index: 6,
         //van_index: 1,
         direction_index: DIRECTION_INDEX.EAST
       },*/
      /*{
         row_index: 2,
         col_index: 8,
         van_index: 1,
         direction_index: DIRECTION_INDEX.SOUTH
       },
      {
         row_index: 5,
         col_index: 4,
         van_index: 1,
         direction_index: DIRECTION_INDEX.SOUTH
       },

       {
         row_index: 5,
         col_index: 4,
         van_index: 1,
         direction_index: DIRECTION_INDEX.SOUTH
       },

      {
         row_index: 4,
         col_index: 2,
         van_index: 1,
         direction_index: DIRECTION_INDEX.WEST
       },

      {
         row_index: 5,
         col_index: 8,
         van_index: 1,
         direction_index: DIRECTION_INDEX.WEST
       },

      {
         row_index: 5,
         col_index: 6,
         van_index: 1,
         direction_index: DIRECTION_INDEX.WEST
       },

       {
         row_index: 10,
         col_index: 4,
         van_index: 1,
         direction_index: DIRECTION_INDEX.WEST
       },

      {
         row_index: 8,
         col_index: 4,
         van_index: 1,
         direction_index: DIRECTION_INDEX.SOUTH
       },

      {
         row_index: 8,
         col_index: 2,
         van_index: 1,
         direction_index: DIRECTION_INDEX.NORTH
       },
      {
         row_index: 7,
         col_index: 2,
         van_index: 1,
         direction_index: DIRECTION_INDEX.NORTH
       },
      {
         row_index: 6,
         col_index: 2,
         van_index: 1,
         direction_index: DIRECTION_INDEX.NORTH
       },
      {
         row_index: 5,
         col_index: 2,
         van_index: 1,
         direction_index: DIRECTION_INDEX.NORTH
       },*/

    ];

    const request: RequestSetOverrideList = {
      tag: RequestTypes.SET_OVERRIDE_LIST,
      overRideList
    };
    this.worker.postMessage(request);
  }

  handleGridStateFromWasm() {
    this.log.debug("Data is now", this.gridState);


    //strip out empty cells
    if (_.isNil(this.gridState)) {
      return;
    }

    this.cells = this.gridState.tiles.map((t, index) => {
      return tileToCellState(this.gridState, t, index);
    });

    const nonEmptyCells: Array<CellData> = this.cells.filter(
      cell => cell.tile.type !== "Empty"
    );

    //this.log.debug("Non empty cells", nonEmptyCells);

    //Only continue processing on initial tick/load
    if (this.gridState.tick > 0) {
      return;
    }

    this.sendOverrideList();

    if (nonEmptyCells.length > 0) {
      this.log.debug("STORING GRID");
      this.gridStorageService.storeGrid(this.gridState);
    }

    const theJSON = JSON.stringify(this.gridState, null, 2);
    this.jsonSaveAs = this.sanitizer.bypassSecurityTrustUrl(
      "data:text/json;charset=UTF-8," + encodeURIComponent(theJSON));


  }


  handleFileSelect(evt) {
    const files = evt.target.files; // FileList object

    // files is a FileList of File objects. List some properties.
    const file: File = files[0];


    const reader = new FileReader();

    reader.onload = () => {
      const fileTextData: string = reader.result as string;

      const savedData: GridState = JSON.parse(fileTextData);

      this.log.debug("Loading saved grid from File", savedData);
      this.loadJsonFromClient(savedData);
    };


    reader.readAsText(file);


  }

  /**
   * From UI
   */
  handleWidthHeightChange() {


    this.numRows = _.toNumber(this.numRows);
    this.numCols = _.toNumber(this.numCols);

    if (_.isNil(this.gridState)) {
      return;
    }

    const loadTemp = this.tempTiles.length !== 0;

    if (!loadTemp) {
      this.tempTiles = this.gridState.tiles.map((t, idx) =>
        tileToCellState(this.gridState, t, idx));
    }

    this.gridState.width = this.numCols;
    this.gridState.height = this.numRows;

    //basically keep loading until the next change
    if (loadTemp)  {
      for (const cell of this.tempTiles) {
        this.gridState.tiles[ cell.row_index * this.gridState.width + cell.col_index ] = cell.tile;
      }
    }

    this.sendGridStateToWasm(this.gridState);
  }


  ngOnInit(): void {

    this.log.debug("ng on init");

    const numSteps = localStorage.getItem(LOCAL_STORAGE_KEY_NUM_STEPS);

    if (!_.isNil(numSteps)) {
      this.numCalcSteps = _.toNumber(numSteps);
    }

    //RustRGBProject/pkg works but not in PyCharm

    this.worker = new Worker('assets/worker.js');
    //this.log.debug("Load", workerPath);
    //this.worker = new EricWorker();

    const requestLoad: RequestWasmLoad = {
      tag: RequestTypes.LOAD_WASM
    };

    this.worker.addEventListener("message", ev => {
      this.log.info("Got message", ev);

      const message: WasmWebWorkerResponse = ev.data;

      switch (message.tag) {
        case ResponseTypes.WASM_LOADED: {
          this.log.debug("Angular got web worker load");
          this.handleWasmLoaded(message);
          break;
        }
        case ResponseTypes.GRID_STATE_LOADED:

          this.log.debug("New grid state data");
          this.gridState = message.data;
          this.handleGridStateFromWasm();
          break;

        case ResponseTypes.BATCH_PROGRESS_MESSAGE:

          let secElapsed = (message.currentMs - message.startedMs) / 1000;
          const minutesElapsed = _.floor( secElapsed / 60);
          secElapsed -= minutesElapsed * 60;

          const failure = !_.isNil(message.success) && !message.success;

          this.progressMessage = `${message.success ? 'Success! ' : ''}${failure ? 'Failure! ' : ''}` +
            `Iteration Count: [${message.stepsCompleted.toLocaleString()}].  ` +
            `min: ${minutesElapsed} secs: ${secElapsed.toFixed(2)}`;

          break;
      }

    });

    this.worker.postMessage(requestLoad);


    //submitButton.addEventListener("click", () => worker.postMessage(textBox.value));

    this.gridMouseMove$.pipe(throttleTime(100)).subscribe(
      (e) => this.handleMouseMove(e)
    );

    this.gridMouseDown$.pipe(
      mergeMap(() => this.gridMouseMove$.pipe(
        //ms
        throttleTime(25),
        takeUntil(this.gridMouseUp$)))
    ).subscribe((event) => {
      this.log.debug(`Drag event ${event.buttons}`);

      this.handleGridClick(event, event.buttons === 2);

    });
  }

  handleGridMouseEvent(mouseEventType: "down" | "up" | "move" | "right" | "left", mouseEvent: MouseEvent): boolean {

    if (this.colors.length === 0) {
      //wasm not loaded
      return false;
    }

    //this.log.debug("mouse event", mouseEventType);
    switch (mouseEventType) {
      case "down":
        this.gridMouseDown$.next(mouseEvent);
        break;
      case "up":
        this.gridMouseUp$.next(mouseEvent);
        break;
      case "move":
        this.gridMouseMove$.next(mouseEvent);
        break;
      case "right":
        this.gridMouseDown$.next(mouseEvent);
        const mouseEventOverride = {
          buttons: 2,
          target: mouseEvent.target, clientX: mouseEvent.clientX, clientY: mouseEvent.clientY
        };
        this.gridMouseMove$.next(mouseEventOverride as any);
        this.gridMouseUp$.next(mouseEvent);

        break;
      case "left":
        this.gridMouseDown$.next(mouseEvent);
        const mouseEventOverride2 = {
          buttons: 1,
          target: mouseEvent.target, clientX: mouseEvent.clientX, clientY: mouseEvent.clientY
        };
        this.gridMouseMove$.next(mouseEventOverride2 as any);
        this.gridMouseUp$.next(mouseEvent);

        break;

    }
    return false;
  }

  onTileClick(t) {
    this.selectedTile = t;
  }

  onColorClick(c) {
    this.selectedColor = c;
  }

  buildConnectionMask() {
    let mask = 0;
    for (let i = 0; i < 4; ++i) {
      if (this.isDirectionOn[i]) {
        mask |= 1 << i ;
      }
    }

    return mask;
  }

  isAllowed(connectionMask: number, direction: number) : boolean {
    return (connectionMask & (1 << direction)) > 0;
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

    const mmRow: number = _.floor(y / this.GRID_SIZE);
    const mmCol = _.floor(x / this.GRID_SIZE);

    const leftPadNumber = (n) => _.toString(n).padStart(2, '0');

    this.mouseMoveRow = leftPadNumber( mmRow );
    this.mouseMoveCol = leftPadNumber(mmCol);
    this.mouseMoveIndex = leftPadNumber(mmRow * this.gridState.width + mmCol);
  }

  handleGridClick(clickEvent: MouseEvent, isRightClick: boolean): boolean {

    //this.log.debug(clickEvent);
    // this.log.debug(clickEvent.target);

    const rect = (clickEvent.target as any).getBoundingClientRect();

    const x = clickEvent.clientX - rect.left; //x position within the element.
    const y = clickEvent.clientY - rect.top;  //y position within the element.

    const colIndex = _.floor(x / this.GRID_SIZE);
    const rowIndex = _.floor(y / this.GRID_SIZE);

    this.log.debug(`Clicked on row ${rowIndex}, col ${colIndex}.  Right click? ${isRightClick}`);

    if (_.isNil(this.gridState)) {
      return false;
    }

    if (isRightClick) {
      const cellIndex = rowIndex * this.numCols + colIndex;
      let tile: TileEnum = this.gridState.tiles[cellIndex];

      //make it a road
      if (tile.type !== "TileRoad") {
        tile = {
          type: "TileRoad",
          has_popper: false,
          connection_mask: this.buildConnectionMask()
        };
      }

      switch (this.selectedThing) {
        case "Van":
          tile.van = {boxes: [null, null, null], color: this.selectedColor.color_index, is_done: false};
          break;
        case "Block":
          tile.block = this.selectedColor.color_index;
          break;
        case "Button":
          tile.button = {
            color: this.selectedColor.color_index,
            is_pressed: !this.selectedIsOpenOrUp
          };
          break;
        case "Popper":
          tile.has_popper = true;
          break;
        case "Clear":
          delete tile.block;
          delete tile.van;
          tile.has_popper = false;
          delete tile.button;
          break;
      }

      this.setGridSquare({row_index: rowIndex, col_index: colIndex, tile});


    } else {
      switch (this.selectedTile) {
        case "Empty":
          this.setGridSquare({
            row_index: rowIndex,
            col_index: colIndex,
            tile: {type: this.selectedTile}
          });
          break;
        case "TileRoad":
          this.setGridSquare({
            row_index: rowIndex, col_index: colIndex, tile: {
              type: this.selectedTile,
              has_popper: false,
              connection_mask: this.buildConnectionMask()
            }
          });
          break;
        case "TileWarehouse":
          this.setGridSquare({
            row_index: rowIndex, col_index: colIndex, tile: {
              type: this.selectedTile,
              color: this.selectedColor.color_index,
              is_filled: false
            }
          });
          break;
        case "TileBridge":
          this.setGridSquare({
            row_index: rowIndex, col_index: colIndex, tile: {
              type: this.selectedTile,
              is_up: this.selectedIsOpenOrUp,
              color: this.selectedColor.color_index,
              connection_mask: this.buildConnectionMask()

            }
          });
          break;
      }
    }

    return false;
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

    return !_.isNil(cell.tile.used_van_index) && !_.isNil(cell.tile.used_van_index[dm.dir_index]);
  }

  getCssColorForButton(button: Button) {
    return this.getCssForColorIndex(button.color);
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

    if (_.isNil(vanIndex) || _.isNil(this.gridState)) {
      return DEFAULT_DM_COLOR;
    }

    return this.getCssForColorIndex(this.gridState.vans[vanIndex].color);
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
    const request: RequestInitCalculations = {
      tag: RequestTypes.INIT_CALCULATIONS
    };

    this.worker.postMessage(request);
  }

  nextCalculateStep(numStepsParam: number) {
    const numSteps = _.toNumber(numStepsParam);

    const request: RequestRunCalculateSteps = {
      tag: RequestTypes.RUN_CALCULATE_STEPS,
      numSteps
    };

    this.worker.postMessage(request);

    this.progressMessage = "Starting/continuing calculations...";
  }

  private sendGridStateToWasm(gridState: GridState) {
    const request: RequestLoadGridState = {
      tag: RequestTypes.LOAD_GRID_STATE,
      gridState

    };

    this.worker.postMessage(request);
  }

  private setGridSquare(cellData: CellData) {
    const request: RequestSetGridSquare = {
      tag: RequestTypes.SET_GRID_SQUARE,
      cellData
    };

    this.worker.postMessage(request);

    this.initCalculations();
  }

  handleNumCalcStepsChange(steps) {
    localStorage.setItem(LOCAL_STORAGE_KEY_NUM_STEPS, steps);
  }

  resetGrid() {

    this.fileInput.nativeElement.value = "";

    const gs: GridState = {
      ...EMPTY_GRID_STATE,
      width: this.numCols,
      height: this.numRows,
      tiles: [],
    };
    for (let row = 0; row < this.numRows; ++row) {
      for (let col = 0; col < this.numCols; ++col) {
        gs.tiles.push({
          type: "Empty",

        });
      }
    }

    this.loadJsonFromClient(gs);
  }

  //if the data comes from the user (a file, local storage, etc.)
  loadJsonFromClient(data: GridState | null) {
    if (!_.isNil(data)) {
      this.log.debug("Loading saved grid from local storage", data);

      //convert cells to tiles
      if (_.isArray((data as any).cells)) {
        data.tiles = ((data as any).cells as Array<CellData>).map(c => c.tile);
      }

      this.sendGridStateToWasm(data);
      this.numRows = data.height;
      this.numCols = data.width;

      this.tempTiles = [];

      this.initCalculations();


      this.sendOverrideList();

      //temp
      //this.nextCalculateStep(100000);
    }
  }

  loadExample(exampleName: string) {
    fetch(`assets/example_levels/${exampleName}.json`).then(response => {
      return response.json();
    }).then(jsonData => {
      this.loadJsonFromClient(jsonData);
    }).catch((error) => {
      this.log.debug(error);
    });

  }
}


function tileToCellState(gridState: GridState, tile: TileEnum, index: number): CellData {

    if (!_.isFinite(index)) {
        throw Error(`Index is not a number: ${index}`);
    }

    const w = gridState.width;

    if (!_.isFinite(w) || w < 0) {
        throw Error(`Width must be >0: ${w}`);
    }

    return {
        tile,
        row_index: _.floor(index / w),
        col_index: index % w
    };
}


const EMPTY_GRID_STATE: GridState = {
  width: 0,
  height: 0,
  vans: [],
  tiles: [],
  tick: -1,
  warehouses_remaining: 0,
  bridges: [],
  buttons: []
};
