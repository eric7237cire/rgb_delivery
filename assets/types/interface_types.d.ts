import { CellData, ChoiceOverride, Color, GridState } from "rgb-solver";
export declare const enum ResponseTypes {
    WASM_LOADED = 0,
    GRID_STATE_LOADED = 1,
    BATCH_PROGRESS_MESSAGE = 2
}
export interface ResponseWasmLoaded {
    tag: ResponseTypes.WASM_LOADED;
    colors: Array<Color>;
}
export interface ResponseDataLoaded {
    tag: ResponseTypes.GRID_STATE_LOADED;
    data: GridState;
}
export interface ResponseProgressMessage {
    tag: ResponseTypes.BATCH_PROGRESS_MESSAGE;
    stepsCompleted: number;
    startedMs: number;
    currentMs: number;
    success?: boolean;
}
export declare type WasmWebWorkerResponse = ResponseWasmLoaded | ResponseDataLoaded | ResponseProgressMessage;
export declare const enum RequestTypes {
    LOAD_GRID_STATE = 0,
    LOAD_WASM = 1,
    SET_GRID_SQUARE = 2,
    INIT_CALCULATIONS = 3,
    RUN_CALCULATE_STEPS = 4,
    SET_OVERRIDE_LIST = 5
}
export interface RequestWasmLoad {
    tag: RequestTypes.LOAD_WASM;
}
export interface RequestLoadGridState {
    tag: RequestTypes.LOAD_GRID_STATE;
    gridState: GridState;
}
export interface RequestSetGridSquare {
    tag: RequestTypes.SET_GRID_SQUARE;
    cellData: CellData;
}
export interface RequestInitCalculations {
    tag: RequestTypes.INIT_CALCULATIONS;
    maxSteps: number;
}
export interface RequestRunCalculateSteps {
    tag: RequestTypes.RUN_CALCULATE_STEPS;
    numSteps: number;
}
export interface RequestSetOverrideList {
    tag: RequestTypes.SET_OVERRIDE_LIST;
    overRideList: Array<ChoiceOverride>;
}
export declare type WasmWebWorkerRequest = RequestWasmLoad | RequestLoadGridState | RequestSetGridSquare | RequestInitCalculations | RequestRunCalculateSteps | RequestSetOverrideList;
