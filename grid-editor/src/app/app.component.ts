import { Component } from '@angular/core';

import loadWasm from '../../../rgb-solver/src/lib.rs';
console.log('I am alive!!!');
loadWasm().then(result => {
  const {add, subtract, multiply} = result.instance.exports;
  console.log('4 + 2 = ', add(4, 2));
  console.log('4 - 2 = ', subtract(4, 2));
  console.log('4 * 2 = ', multiply(4, 2));
});

/*
const wasm = imp rt("../../../rgb-solver/pkg/rgb_solver");

wasm.then(module => {
  // won't typecheck if yourlib does not expose the run function
  module.greet();
});*/

class Color {
  css_color: string;
  label: string;

  constructor(_css_class, _label) {
    this.css_color = _css_class;
    this.label = _label;
  }
}

class TileType {
  key: string;
  label: string;

  constructor(_k, _l) {
    this.key = _k;
    this.label = _l;
  }
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.styl']
})
export class AppComponent {
  title = 'grid-editor';
  grid_size = 40;

  num_cols = 10;
  num_rows = 10;

  colors = [ new Color("rgb(255,0,0)","Red"), new Color("rgb(255,255,0)","Yellow"),
  new Color("rgb(50,50,255)", "Blue"), new Color("rgb(230,230,230)", "White") ] ;

  tiles = [ new TileType("R", "Road"),
  new TileType("B", "Block"),
  new TileType("W", "Warehouse"),
  new TileType("V", "Van") ]

  selectedColor = this.colors[0];
  selectedTile = this.tiles[0];

onTileClick(t) {
  this.selectedTile = t;
}

  onColorClick(c) {
    this.selectedColor = c;
  }
}
