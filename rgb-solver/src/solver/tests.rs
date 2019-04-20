

#[cfg(test)]
mod tests {

    mod test_data {
        //
        // d:\Miniconda3\envs\scripts\python -m cogapp -r "D:\git\rgb_delivery\rgb-solver\src\solver\tests.rs"

        /* [[[cog

from pathlib import Path
#LEVEL_DIR = Path(r'D:\git\rgb_delivery\levels')
LEVEL_DIR = Path(r'/mnt/e/git/rgb_delivery/levels')

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
        pub (crate) const TEST_DATA_BIRMINGHAM_G10: &str = r#"
                {
  "width": 11,
  "height": 11,
  "tiles": [
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileWarehouse",
      "color": 4,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 1
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileWarehouse",
      "color": 3,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 3,
        "is_done": false
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 3
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 4
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 4,
        "is_done": false
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 1,
        "is_done": false
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 1
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 1
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 1,
        "is_done": false
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 1
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    }
  ],
  "tick": 0,
  "vans": [],
  "bridges": [],
  "buttons": [],
  "warehouses_remaining": 0
}
"#;
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
  "tiles": [
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 4,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 1,
        "is_done": false
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 4,
        "is_done": false
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 1
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 4
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 1
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    }
  ],
  "tick": 0,
  "vans": [
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 1,
      "is_done": false
    },
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 4,
      "is_done": false
    }
  ],
  "bridges": [],
  "buttons": [],
  "warehouses_remaining": 3
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_BOSTON_N5: &str = r#"
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
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 6,
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
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 7
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
        "type": "TileRoad",
        "used_mask": 0
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
        "type": "TileRoad",
        "used_mask": 0
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
  "tick": 0,
  "vans": []
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_BOSTON_N6: &str = r#"
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
          "label": "Blue",
          "red": 50,
          "green": 50,
          "blue": 255,
          "color_index": 3
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
        "type": "TileRoad",
        "used_mask": 0
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
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0
      },
      "row_index": 7,
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
            "label": "Blue",
            "red": 50,
            "green": 50,
            "blue": 255,
            "color_index": 3
          },
          "is_done": false
        }
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
  ],
  "tick": 0,
  "vans": [
    {
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
        pub (crate) const TEST_DATA_DALLAS_C10: &str = r#"
                {
  "width": 11,
  "height": 11,
  "tiles": [
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 4,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 1,
        "is_done": false
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 2
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 1
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 2,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 4,
        "is_done": false
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 4
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 2,
        "is_done": false
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    }
  ],
  "tick": 0,
  "vans": [
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 1,
      "is_done": false
    },
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 4,
      "is_done": false
    },
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 2,
      "is_done": false
    }
  ],
  "bridges": [],
  "buttons": [],
  "warehouses_remaining": 3
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_EASY1: &str = r#"
                {
  "width": 5,
  "height": 5,
  "tiles": [
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 1,
        "is_done": false
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 1
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    }
  ],
  "tick": 0,
  "vans": [
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 1,
      "is_done": false
    }
  ],
  "bridges": [],
  "buttons": [],
  "warehouses_remaining": 1
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
#[allow(dead_code)]
        pub (crate) const TEST_DATA_LIVERPOOL_E10: &str = r#"
                {
  "width": 11,
  "height": 11,
  "tiles": [
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 2,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_tick": null,
      "is_up": true,
      "color": 5
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_tick": null,
      "is_up": false,
      "color": 5
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "button": {
        "is_pressed": true,
        "color": 5
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "button": {
        "is_pressed": false,
        "color": 5
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 1
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 2
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 1,
        "is_done": false
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 2,
        "is_done": false
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    }
  ],
  "tick": 0,
  "vans": [
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 1,
      "is_done": false
    },
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 2,
      "is_done": false
    }
  ],
  "bridges": [
    {
      "used_van_index": null,
      "used_tick": null,
      "is_up": true,
      "color": 5
    },
    {
      "used_van_index": null,
      "used_tick": null,
      "is_up": false,
      "color": 5
    }
  ],
  "buttons": [
    {
      "is_pressed": true,
      "color": 5
    },
    {
      "is_pressed": false,
      "color": 5
    }
  ],
  "warehouses_remaining": 2
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_LONDON_H10: &str = r#"
                {
  "width": 11,
  "height": 11,
  "tiles": [
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": true,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": true,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 2,
        "is_done": false
      }
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "block": 1
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileWarehouse",
      "color": 2,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": true,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "block": 1
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": true,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": true,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 1,
        "is_done": false
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "block": 1
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "block": 2
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": true,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "block": 1
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 1,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    }
  ],
  "tick": 0,
  "vans": [
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 2,
      "is_done": false
    },
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 1,
      "is_done": false
    }
  ],
  "bridges": [],
  "buttons": [],
  "warehouses_remaining": 5
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_LONDON_H2: &str = r#"
                {
  "width": 11,
  "height": 11,
  "tiles": [
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 6
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "button": {
        "is_pressed": false,
        "color": 5
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 3,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 5
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 6
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 8
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "button": {
        "is_pressed": false,
        "color": 8
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 8
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 7
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 7
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "button": {
        "is_pressed": false,
        "color": 7
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 3,
        "is_done": false
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "block": 3
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null,
      "button": {
        "is_pressed": false,
        "color": 6
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "has_popper": false,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "used_popper_tick": null
    }
  ],
  "tick": 0,
  "vans": [],
  "bridges": [],
  "buttons": [],
  "warehouses_remaining": 1
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_MANCHESTER_F10: &str = r#"
                {
  "width": 11,
  "height": 11,
  "tiles": [
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": false,
      "color": 5
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "button": {
        "is_pressed": false,
        "color": 5
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 5
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "block": 3
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": false,
      "color": 5
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileWarehouse",
      "color": 3,
      "is_filled": false
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "van": {
        "boxes": [
          null,
          null,
          null
        ],
        "color": 3,
        "is_done": false
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 5
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": false,
      "color": 5
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 5
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "button": {
        "is_pressed": false,
        "color": 5
      }
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": true,
      "color": 5
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileBridge",
      "used_van_index": null,
      "used_mask": 0,
      "used_tick": null,
      "is_up": false,
      "color": 5
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ],
      "button": {
        "is_pressed": true,
        "color": 5
      }
    },
    {
      "type": "TileRoad",
      "used_mask": 0,
      "used_van_index": [
        null,
        null,
        null,
        null
      ],
      "used_tick": [
        null,
        null,
        null,
        null
      ]
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    },
    {
      "type": "Empty"
    }
  ],
  "tick": 0,
  "vans": [],
  "bridges": [],
  "buttons": [],
  "warehouses_remaining": 0
}
"#;
#[allow(dead_code)]
        pub (crate) const TEST_DATA_SAN_JOSE_D10: &str = r#"
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
        "type": "TileWarehouse",
        "color": 1,
        "is_filled": false
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
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 2,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 2,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 4,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ],
        "block": 2
      },
      "row_index": 4,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 4,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 4,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 4,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ],
        "block": 4
      },
      "row_index": 4,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 5,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": 1,
        "is_filled": false
      },
      "row_index": 5,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ],
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": 0,
          "is_done": false
        }
      },
      "row_index": 6,
      "col_index": 0
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 6,
      "col_index": 1
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 6,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 6,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 6,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 6,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 6,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ],
        "block": 1
      },
      "row_index": 6,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 6,
      "col_index": 8
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 6,
      "col_index": 9
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ],
        "van": {
          "boxes": [
            null,
            null,
            null
          ],
          "color": 1,
          "is_done": false
        }
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
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ],
        "block": 1
      },
      "row_index": 7,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": 2,
        "is_filled": false
      },
      "row_index": 7,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 7,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileWarehouse",
        "color": 4,
        "is_filled": false
      },
      "row_index": 7,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 8,
      "col_index": 2
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 8,
      "col_index": 3
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 8,
      "col_index": 4
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 8,
      "col_index": 5
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 8,
      "col_index": 6
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
      },
      "row_index": 8,
      "col_index": 7
    },
    {
      "tile": {
        "type": "TileRoad",
        "used_mask": 0,
        "used_van_index": [
          null,
          null,
          null,
          null
        ],
        "used_tick": [
          null,
          null,
          null,
          null
        ]
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
  "tick": 0,
  "vans": [
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 0,
      "is_done": false
    },
    {
      "boxes": [
        null,
        null,
        null
      ],
      "color": 1,
      "is_done": false
    }
  ]
}
"#;
// --[[[end]]]
    }
    use crate::solver::struct_defs::{ Universe};
    use crate::solver::grid_state:: {GridState};
    use self::test_data::*;
    use serde_json;

    #[cfg( target_arch = "x86_64")]
    #[test]
    fn test_add() {
        assert_eq!(3, 3);


    }

    #[test]
    fn test_easy1() {
        let universe_data: GridState = serde_json::from_str(TEST_DATA_EASY1).unwrap();

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

    #[test]
    fn test_boston_n10() {
        //
        let universe_data: GridState = serde_json::from_str(TEST_DATA_BOSTON_N10).unwrap();

        let mut universe = Universe::new(universe_data.width, universe_data.height);

        universe.data = universe_data;

        universe.init_calculate();

        for i in 0..15000 {
            let cd = universe.process_queue_item();
            assert!(cd.is_some(), "Failed after {}", i);

            if universe.success.is_some() {
                break;
            }
        }

        assert!(universe.success.is_some());
    }

    #[test]
    fn test_dallas_10() {
        //
        let universe_data: GridState = serde_json::from_str(TEST_DATA_DALLAS_C10).unwrap();

        let mut universe = Universe::new(universe_data.width, universe_data.height);

        universe.data = universe_data;

        universe.init_calculate();

        for i in 0..150000 {
            let cd = universe.process_queue_item();
            assert!(cd.is_some(), "Failed after {}", i);

            if universe.success.is_some() {
                break;
            }
        }

        assert!(universe.success.is_some());
    }

    #[test]
    fn test_liverpool_10() {
        //
        let universe_data: GridState = serde_json::from_str(TEST_DATA_LIVERPOOL_E10).unwrap();

        let mut universe = Universe::new(universe_data.width, universe_data.height);

        universe.data = universe_data;

        universe.init_calculate();

        for i in 0..150000 {
            let cd = universe.process_queue_item();
            assert!(cd.is_some(), "Failed after {}", i);

            if universe.success.is_some() {
                break;
            }
        }

        assert!(universe.success.is_some());
    }

    #[test]
    fn test_birmingham_g10() {
        //
        let universe_data: GridState = serde_json::from_str(TEST_DATA_BIRMINGHAM_G10).unwrap();

        let mut universe = Universe::new(universe_data.width, universe_data.height);

        universe.data = universe_data;

        universe.init_calculate();

        for i in 0..150000 {
            let cd = universe.process_queue_item();
            assert!(cd.is_some(), "Failed after {}", i);

            if universe.success.is_some() {
                break;
            }
        }

        assert!(universe.success.is_some());
    }

    //Poppers!
    #[test]
    fn test_london_h10() {
        //
        let universe_data: GridState = serde_json::from_str(TEST_DATA_LONDON_H10).unwrap();

        let mut universe = Universe::new(universe_data.width, universe_data.height);

        universe.data = universe_data;

        universe.init_calculate();

        for i in 0..150000 {
            let cd = universe.process_queue_item();
            assert!(cd.is_some(), "Failed after {}", i);

            if universe.success.is_some() {
                break;
            }
        }

        assert!(universe.success.is_some());
    }
}