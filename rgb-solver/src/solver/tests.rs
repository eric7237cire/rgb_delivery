

#[cfg(test)]
mod tests {

    mod test_data {
        //
        // d:\Miniconda3\envs\scripts\python -m cogapp -r "D:\git\rgb_delivery\rgb-solver\src\solver\tests.rs"

        /* [[[cog

from pathlib import Path
LEVEL_DIR = Path(r'D:\git\rgb_delivery\levels')

for json_file in LEVEL_DIR.glob('*.json'):
    with open(json_file, 'r') as f:
        json_data = f.read()

        var_suffix = json_file.stem.replace(' ', '_').upper()

        cog.outl(f"""#[allow(dead_code)]
        pub (crate) const TEST_DATA_{var_suffix}: &str = r#"
                {json_data}
"#;""")
]]] */
#[allow(dead_code)]
        pub (crate) const TEST_DATA_BOSTON_N1: &str = r#"
                {
  "width": 11,
  "height": 11,
  "cells": [
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 0,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 1,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 4,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "Red",
            "red": 255,
            "green": 0,
            "blue": 0,
            "color_index": 1
          },
          "is_done": false
        }
      },
      "row_index": 9,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 10
    }
  ]
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_BOSTON_N10: &str = r#"
                {
  "width": 11,
  "height": 11,
  "cells": [
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Green",
          "red": 0,
          "green": 255,
          "blue": 0,
          "color_index": 4
        },
        "is_filled": false
      },
      "row_index": 0,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 0,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "Red",
            "red": 255,
            "green": 0,
            "blue": 0,
            "color_index": 1
          },
          "is_done": false
        }
      },
      "row_index": 1,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "Green",
            "red": 0,
            "green": 255,
            "blue": 0,
            "color_index": 4
          },
          "is_done": false
        }
      },
      "row_index": 1,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 3,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 4,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Green",
          "red": 0,
          "green": 255,
          "blue": 0,
          "color_index": 4
        }
      },
      "row_index": 6,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 8,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 10
    }
  ]
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_CALGARY_N1: &str = r#"
                {
  "width": 11,
  "height": 11,
  "cells": [
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "White",
            "red": 230,
            "green": 230,
            "blue": 230,
            "color_index": 0
          },
          "is_done": false
        }
      },
      "row_index": 0,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 0,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 0,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Yellow",
          "red": 255,
          "green": 255,
          "blue": 0,
          "color_index": 2
        },
        "is_filled": false
      },
      "row_index": 1,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 10
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 10
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Yellow",
          "red": 255,
          "green": 255,
          "blue": 0,
          "color_index": 2
        }
      },
      "row_index": 5,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 5,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Blue",
          "red": 50,
          "green": 50,
          "blue": 255,
          "color_index": 3
        },
        "is_filled": false
      },
      "row_index": 6,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 6,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Blue",
          "red": 50,
          "green": 50,
          "blue": 255,
          "color_index": 3
        }
      },
      "row_index": 9,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 9,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 10
    }
  ]
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_EASY1: &str = r#"
                {
  "width": 5,
  "height": 5,
  "cells": [
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "Red",
            "red": 255,
            "green": 0,
            "blue": 0,
            "color_index": 1
          },
          "is_done": false
        }
      },
      "row_index": 0,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 3,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 4
    }
  ]
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_EASY2: &str = r#"
                {
  "width": 8,
  "height": 8,
  "cells": [
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "Red",
            "red": 255,
            "green": 0,
            "blue": 0,
            "color_index": 1
          },
          "is_done": false
        }
      },
      "row_index": 0,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 3,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 7
    }
  ]
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_EASY3: &str = r#"
                {
  "width": 8,
  "height": 8,
  "cells": [
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "Red",
            "red": 255,
            "green": 0,
            "blue": 0,
            "color_index": 1
          },
          "is_done": false
        }
      },
      "row_index": 0,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 1,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 2,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Yellow",
          "red": 255,
          "green": 255,
          "blue": 0,
          "color_index": 2
        },
        "is_filled": false
      },
      "row_index": 4,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 5,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Yellow",
          "red": 255,
          "green": 255,
          "blue": 0,
          "color_index": 2
        }
      },
      "row_index": 5,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 7
    }
  ]
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_EASY4: &str = r#"
                {
  "width": 11,
  "height": 11,
  "cells": [
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "Red",
            "red": 255,
            "green": 0,
            "blue": 0,
            "color_index": 1
          },
          "is_done": false
        }
      },
      "row_index": 3,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 4,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 5,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Yellow",
          "red": 255,
          "green": 255,
          "blue": 0,
          "color_index": 2
        },
        "is_filled": false
      },
      "row_index": 5,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "Yellow",
            "red": 255,
            "green": 255,
            "blue": 0,
            "color_index": 2
          },
          "is_done": false
        }
      },
      "row_index": 6,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Yellow",
          "red": 255,
          "green": 255,
          "blue": 0,
          "color_index": 2
        }
      },
      "row_index": 6,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 7,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 8,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 8,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 10
    }
  ]
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_EASY5: &str = r#"
                {
  "width": 11,
  "height": 11,
  "cells": [
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 2,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 2,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        },
        "is_filled": false
      },
      "row_index": 2,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 3,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 3,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "block": {
          "label": "Red",
          "red": 255,
          "green": 0,
          "blue": 0,
          "color_index": 1
        }
      },
      "row_index": 5,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": {
            "label": "Red",
            "red": 255,
            "green": 0,
            "blue": 0,
            "color_index": 1
          },
          "is_done": false
        }
      },
      "row_index": 6,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 10
    }
  ],
  "tick": 0
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_EMPTY: &str = r#"
                {
  "width": 11,
  "height": 11,
  "cells": [
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 0,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 1,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 2,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 3,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 4,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 5,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 6,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 7,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 8,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 9,
      "col_index": 10
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 0
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 1
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 2
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 3
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 4
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 5
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 6
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 7
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 8
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 9
    },
    {
      "tile": {
        "type": "Empty"
      },
      "row_index": 10,
      "col_index": 10
    }
  ],
  "tick": 0
}
"#;
// --[[[end]]]
    }
    use crate::solver::struct_defs::{UniverseData, Universe};
    use self::test_data::*;
    use serde_json;

    #[cfg( target_arch = "x86_64")]
    #[test]
    fn test_add() {
        assert_eq!(3, 3);


    }

    #[test]
    fn test_easy1() {
        let universe_data: UniverseData = serde_json::from_str(TEST_DATA_EASY1).unwrap();

        let mut universe = Universe::new(universe_data.width, universe_data.height);

        universe.data = universe_data;

        universe.init_calculate();

        for i in 0..1000 {
            let cd = universe.process_queue_item();
            assert!(cd.is_some(), "Failed after {}", i);

            if universe.success.is_some() {
                break;
            }
        }

        assert!(universe.success.is_some());
    }


}