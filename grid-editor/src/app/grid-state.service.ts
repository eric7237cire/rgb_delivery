import {CellData, Color, GridState, TileEnum, Universe} from "rgb-solver";
import {Injectable} from "@angular/core";
import * as _ from "lodash";
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

  public gridState$: BehaviorSubject<GridState> = new BehaviorSubject<GridState>(EMPTY_GRID_STATE);

  get gridState(): GridState  {
    return this.gridState$.getValue();
  }


  static tileToCellState(gridState: GridState, tile: TileEnum, index: number): CellData {

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





}
