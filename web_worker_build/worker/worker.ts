import {CellData, ChoiceOverride, Color, GridState, TileEnum, Universe} from "rgb-solver";
import * as _ from "lodash";
import {
    RequestTypes,
    ResponseDataLoaded,
    ResponseTypes,
    ResponseWasmLoaded,
    WasmWebWorkerRequest
} from "./interface_types";

export type WASM_TYPE = typeof import ('rgb-solver');


let g_worker: RgbWasmWorker;
const ctx: Worker = self as any;

ctx.addEventListener("message", ev => {

    let requestMessage: WasmWebWorkerRequest = ev.data;

    console.log("Got message", requestMessage.tag);

    switch (requestMessage.tag) {
        case RequestTypes.LOAD_WASM: {
            console.log("Loading wasm");

            //npm link rgb-solver
            import("rgb-solver").then(wasm => {

                g_worker = new RgbWasmWorker(wasm);
                let readyMsg: ResponseWasmLoaded = {
                    tag: ResponseTypes.WASM_LOADED,
                    colors: wasm.get_colors()
                };


                ctx.postMessage(readyMsg);
            });
            break;
        }
        case RequestTypes.LOAD_GRID_STATE: {
            g_worker.loadGridState(requestMessage.gridState);
            break;
        }
        case RequestTypes.SET_GRID_SQUARE: {
            g_worker.setGridSquare(requestMessage.cellData);
            break;
        }
        case RequestTypes.INIT_CALCULATIONS: {
            g_worker.universe.init_calculate();
            g_worker.reloadGridData();
            break;
        }
        case RequestTypes.RUN_CALCULATE_STEPS: {
            let data = g_worker.universe.next_batch_calculate(requestMessage.numSteps);

            if (_.isNil(data)) {
              console.log("Null grid state after batch");
            } else {
              let dataLoadedMessage: ResponseDataLoaded = {
                    tag: ResponseTypes.GRID_STATE_LOADED,
                    data
                };

                ctx.postMessage(dataLoadedMessage);
            }

            break;
        }
        case RequestTypes.SET_OVERRIDE_LIST: {
            console.log("Setting Overrides", requestMessage.overRideList);
            g_worker.universe.set_overrides(requestMessage.overRideList);
            break;
        }

    }


    //console.log("data", u.get_data());


});

class RgbWasmWorker {

    //wasm: WASM_TYPE;

    universe: Universe;

    constructor(private wasm: WASM_TYPE) {
        this.universe = this.wasm.Universe.new(3, 3);
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

    /**
     * Loads from local storage / file.
     *
     * Not for calculations
     *
     */
    loadGridState(jsonData: GridState) {
        console.log("Loading json data", jsonData);

        this.universe = this.wasm.Universe.new(jsonData.height, jsonData.width);

        console.log("Universe initial data", this.universe.get_data());

        //Giving web assembly the loaded data
        if (!_.isNil(jsonData) && _.isArray(jsonData.tiles)) {
            jsonData.tiles.forEach((tile, idx) => {

                const cellData = RgbWasmWorker.tileToCellState(jsonData, tile, idx);
                this.setGridSquare(cellData, false);


            });
        }

        this.universe.init_calculate();

        //get correct form back from wasm
        this.reloadGridData();
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

            let is_open = (cellData.tile as any)["is_open"];
            if (!_.isNil(is_open)) {
                cellData.tile.is_up = is_open;
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

    reloadGridData() {
        const data = this.universe.get_data();

        if (_.isNil(data)) {
            console.log("Data is null");
        } else {

            let dataLoadedMessage: ResponseDataLoaded = {
                tag: ResponseTypes.GRID_STATE_LOADED,
                data
            };

            ctx.postMessage(dataLoadedMessage);

        }
    }
}