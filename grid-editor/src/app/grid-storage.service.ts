import { Injectable } from '@angular/core';
//import {GridState} from "rgb-solver";
import * as _ from "lodash";
import {GridState} from "web_worker";

@Injectable({
  providedIn: 'root'
})
export class GridStorageService {

  private readonly  STORAGE_KEY = "universeData";

  constructor() {

  }

  public storeGrid(data: GridState) {


    // Put the object into storage
    localStorage.setItem(this.STORAGE_KEY, JSON.stringify(data));

  }

  public loadGrid(): GridState | null {
    // Retrieve the object from storage
    const retrievedObject = localStorage.getItem(this.STORAGE_KEY);

    if (_.isNil(retrievedObject)) {
      console.log("local storage is empty");
      return null;
    }

    try {
      const obj = JSON.parse(retrievedObject);
      console.log("Loaded", obj);

      return obj;
    } catch (e) {
      console.log("Unable to load", e);
    }

    return null;

  }


}
