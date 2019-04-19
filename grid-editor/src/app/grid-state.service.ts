import {CellData, Color, GridState, Universe} from "rgb-solver";
import {Injectable} from "@angular/core";
import * as _ from "lodash";
import {WASM_TYPE} from "./app.component";
import {BehaviorSubject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class GridStateService {

  public universe: Universe;

  public gridState$: BehaviorSubject<GridState> = new BehaviorSubject<GridState>(null);

  get gridState(): GridState {
    return this.gridState$.getValue();
  }

  loadGridState(jsonData: GridState, wasm: WASM_TYPE) {
    console.log("Loading json data", jsonData);

    this.universe = wasm.Universe.new(jsonData.height, jsonData.width);

    console.log("Universe initial data", this.universe.get_data());

    //Giving web assembly the loaded data
    if (!_.isNil(jsonData) && _.isArray(jsonData.cells)) {
      jsonData.cells.forEach((cellData: CellData) => {


        this.setGridSquare(cellData, false);


      });
    }

    //get correct form back from wasm
    this.reloadGridData();
  }

  reloadGridData() {
      this.gridState$.next(this.universe.get_data() );
    }

  /**
   * Cleans data (colors), then transfers to WASM
   */
  setGridSquare(cellData: CellData, refresh_data: boolean = true) {

    //Correct colors
    if (cellData.tile.type == "TileRoad") {
      if (!_.isNil(cellData.tile.block) && !_.isNumber(cellData.tile.block)) {
        //assume its a color
        cellData.tile.block = (<Color><any>cellData.tile.block).color_index;
      }

      if (!_.isNil(cellData.tile.van)) {

        if (!_.isNumber(cellData.tile.van.color)) {
          //assume its a color
          cellData.tile.van.color = (<Color><any>cellData.tile.van.color).color_index;
        }

        cellData.tile.van.boxes.map(box => {
          if (_.isNil(box) || _.isNumber(box)) {
            return box;
          } else {
            //assume its a color
            (<Color><any>box).color_index;
          }


        });
      }
    } else if (cellData.tile.type == "TileWarehouse") {
      if (!_.isNil(cellData.tile.color) && !_.isNumber(cellData.tile.color)) {
        //assume its a color
        cellData.tile.color = (<Color><any>cellData.tile.color).color_index;
      }
    }

    try {
      this.universe.set_square(cellData);

      if (refresh_data) {
        this.reloadGridData();
      }
    } catch (e) {
      console.log("Error with", cellData);
    }


  }
}
