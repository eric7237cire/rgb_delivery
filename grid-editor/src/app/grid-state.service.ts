import {CellData, Color, GridState, TileEnum, Universe} from "rgb-solver";
import {Injectable} from "@angular/core";
import * as _ from "lodash";
import {WASM_TYPE} from "./app.component";
import {BehaviorSubject} from "rxjs";

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

@Injectable({
  providedIn: 'root'
})
export class GridStateService {

  public universe: Universe;

  public gridState$: BehaviorSubject<GridState> = new BehaviorSubject<GridState>(EMPTY_GRID_STATE);

  static tileToCellState(gridState: GridState, tile: TileEnum, index: number): CellData {

    if (!_.isFinite(index)) {
      throw Error(`Index is not a number: ${index}`);
    }

    const w = gridState.width;

    if (!_.isFinite(w) || w < 0 ) {
      throw Error(`Width must be >0: ${w}`);
    }

    return {
        tile,
        row_index: _.floor(index / w),
        col_index: index % w
      };
  }

  get gridState(): GridState  {
    return this.gridState$.getValue();
  }

  /**
   * Loads from local storage / file.
   *
   * Not for calculations
   *
   */
  loadGridState(jsonData: GridState, wasm: WASM_TYPE) {
    console.log("Loading json data", jsonData);

    this.universe = wasm.Universe.new(jsonData.height, jsonData.width);

    console.log("Universe initial data", this.universe.get_data());

    //Giving web assembly the loaded data
    if (!_.isNil(jsonData) && _.isArray(jsonData.tiles)) {
      jsonData.tiles.forEach((tile, idx) => {

        const cellData = GridStateService.tileToCellState(jsonData, tile, idx);
        this.setGridSquare(cellData, false);


      });
    }

    this.universe.init_calculate();

    //get correct form back from wasm
    this.reloadGridData();
  }



  reloadGridData() {
    const data = this.universe.get_data();

    if (_.isNil(data)) {
      console.log("Data is null");
    } else {

      this.gridState$.next(data);

    }
  }

  /**
   * Cleans data (colors), then transfers to WASM
   */
  setGridSquare(cellData: CellData, refreshData: boolean = true) {

    //Correct colors
    if (cellData.tile.type === "TileRoad") {
      if (!_.isNil(cellData.tile.block) && !_.isNumber(cellData.tile.block)) {
        //assume its a color
        cellData.tile.block = (cellData.tile.block as Color).color_index;
      }

      if (!_.isNil(cellData.tile.van) && !_.isNil(cellData.tile.van.boxes)) {

        if (!_.isNumber(cellData.tile.van.color)) {
          //assume its a color
          cellData.tile.van.color = (cellData.tile.van.color as Color).color_index;
        }

        cellData.tile.van.boxes.map(box => {
          if (_.isNil(box) || _.isNumber(box)) {
            return box;
          } else {
            //assume its a color
            return (box as Color).color_index;
          }


        });
      }
    } else if (cellData.tile.type === "TileWarehouse") {
      if (!_.isNil(cellData.tile.color) && !_.isNumber(cellData.tile.color)) {
        //assume its a color
        cellData.tile.color = (cellData.tile.color as Color).color_index;
      }
    } else if (cellData.tile.type === "TileBridge") {
      if (!_.isNil(cellData.tile["is_open"])) {
        cellData.tile.is_up = cellData.tile["is_open"];
      }
    }

    try {
      //console.log("Setting square", cellData);
      this.universe.set_square(cellData);

      if (refreshData) {
        this.reloadGridData();
      }
    } catch (e) {
      console.log("Error with", cellData);
    }


  }
}
