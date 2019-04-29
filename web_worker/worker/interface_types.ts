import {CellData, ChoiceOverride, Color, GridState} from "rgb-solver";

//use const enum to inline
export const enum ResponseTypes {
    WASM_LOADED,
    GRID_STATE_LOADED,
    BATCH_PROGRESS_MESSAGE
}
export interface  ResponseWasmLoaded  {
    tag: ResponseTypes.WASM_LOADED;
    colors: Array<Color>;
}

export interface ResponseDataLoaded  {
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

export type WasmWebWorkerResponse = ResponseWasmLoaded | ResponseDataLoaded | ResponseProgressMessage;


export const enum RequestTypes {
    LOAD_GRID_STATE,
    LOAD_WASM,
    SET_GRID_SQUARE,
    INIT_CALCULATIONS,
    RUN_CALCULATE_STEPS,
    SET_OVERRIDE_LIST
}

export interface  RequestWasmLoad  {
    tag: RequestTypes.LOAD_WASM;
}

export interface  RequestLoadGridState  {
    tag: RequestTypes.LOAD_GRID_STATE;
    gridState: GridState;
}

export interface  RequestSetGridSquare  {
    tag: RequestTypes.SET_GRID_SQUARE;
    cellData: CellData;
}

export interface RequestInitCalculations  {
    tag: RequestTypes.INIT_CALCULATIONS;
}

export interface RequestRunCalculateSteps  {
    tag: RequestTypes.RUN_CALCULATE_STEPS;
    numSteps: number;
}

export interface RequestSetOverrideList  {
    tag: RequestTypes.SET_OVERRIDE_LIST;
    overRideList: Array<ChoiceOverride>;
}

export type WasmWebWorkerRequest = RequestWasmLoad | RequestLoadGridState |
    RequestSetGridSquare  | RequestInitCalculations | RequestRunCalculateSteps |
    RequestSetOverrideList;
