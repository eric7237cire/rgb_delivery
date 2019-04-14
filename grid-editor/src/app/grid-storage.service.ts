import { Injectable } from '@angular/core';
import {UniverseData} from "rgb-solver";

@Injectable({
  providedIn: 'root'
})
export class GridStorageService {

  private readonly  STORAGE_KEY = "universeData";

  constructor() {

  }

  public storeGrid(data: UniverseData) {


    // Put the object into storage
    localStorage.setItem(this.STORAGE_KEY, JSON.stringify(data));

  }

  public loadGrid() : UniverseData {
    // Retrieve the object from storage
    var retrievedObject = localStorage.getItem(this.STORAGE_KEY);

    let obj = null;
    try {
      obj = JSON.parse(retrievedObject);
    } catch(e) {
      console.log("Unable to load", e);
    }

    console.log("Loaded",obj);

    return obj;
  }


}
