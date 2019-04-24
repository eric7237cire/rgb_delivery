

#[cfg(test)]
mod tests {

    mod test_data {
        //
        // d:\Miniconda3\envs\scripts\python -m cogapp -r "D:\git\rgb_delivery\rgb-solver\src\solver\tests.rs"

        /* [[[cog

from pathlib import Path
LEVEL_DIR = Path(r'D:\git\rgb_delivery\levels\levels_to_unit_test')
#LEVEL_DIR = Path(r'/mnt/e/git/rgb_delivery/levels/levels_to_unit_test')

for json_file in LEVEL_DIR.glob('*.json'):
    with open(json_file, 'r') as f:
        json_data = f.read()

        var_suffix = json_file.stem.replace(' ', '_').upper()

        cog.outl(f"""
        pub (crate) const TEST_DATA_{var_suffix}: &str = r#"
                {json_data}
"#;""")
]]] */

        pub (crate) const TEST_DATA_AUSTIN_Q3: &str = r#"
                {
  "width": 11,
  "height": 11,
  "tiles": [
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
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
      "type": "Empty"
    },
    {
      "type": "TileRoad",
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
        "color": 0,
        "is_done": false
      }
    },
    {
      "type": "TileRoad",
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
      "type": "Empty"
    },
    {
      "type": "TileRoad",
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
      "color": 2,
      "is_filled": false
    },
    {
      "type": "Empty"
    },
    {
      "type": "TileRoad",
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
      "color": 4,
      "is_filled": false
    },
    {
      "type": "TileRoad",
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
      "block": 4
    },
    {
      "type": "TileRoad",
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
    }
  ],
  "bridges": [],
  "buttons": [],
  "warehouses_remaining": 4
}
"#;

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

        pub (crate) const TEST_DATA_STARTONLYONE: &str = r#"
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
        "color": 0,
        "is_done": false
      }
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
// --[[[end]]]
    }
    use crate::solver::universe::{ Universe};
    use crate::solver::grid_state:: {GridState};
    use self::test_data::*;
    use serde_json;


    /* [[[cog

import re

def remove_double_newlines(s):
    #return s
    return re.sub(r'\n', ' \n', s)

for json_file in LEVEL_DIR.glob('*.json'):

    test_name = json_file.stem.replace(' ', '_').lower()
    constant_name = json_file.stem.replace(' ', '_').upper()

    cog.out(remove_double_newlines(f"""
//************************************************************************************************/
    #[test]
    fn test_{test_name}() {{
        run_level(TEST_DATA_{constant_name})
    }}"""))
]]] */
 
//************************************************************************************************/ 
    #[test] 
    fn test_austin_q3() { 
        run_level(TEST_DATA_AUSTIN_Q3) 
    } 
//************************************************************************************************/ 
    #[test] 
    fn test_birmingham_g10() { 
        run_level(TEST_DATA_BIRMINGHAM_G10) 
    } 
//************************************************************************************************/ 
    #[test] 
    fn test_boston_n10() { 
        run_level(TEST_DATA_BOSTON_N10) 
    } 
//************************************************************************************************/ 
    #[test] 
    fn test_dallas_c10() { 
        run_level(TEST_DATA_DALLAS_C10) 
    } 
//************************************************************************************************/ 
    #[test] 
    fn test_liverpool_e10() { 
        run_level(TEST_DATA_LIVERPOOL_E10) 
    } 
//************************************************************************************************/ 
    #[test] 
    fn test_london_h10() { 
        run_level(TEST_DATA_LONDON_H10) 
    } 
//************************************************************************************************/ 
    #[test] 
    fn test_startonlyone() { 
        run_level(TEST_DATA_STARTONLYONE) 
    }
    // --[[[end]]]

    fn run_level(data: &str) {
        let universe_data: GridState = serde_json::from_str(data).unwrap();

        let mut universe = Universe::new(universe_data.width, universe_data.height);

        universe.data = universe_data;

        universe.init_calculate();

        for i in 0..150000 {
            let cd = universe.process_queue_item();
            assert!(cd.is_some(), "Failed after {}", i);

            if universe.success_state.is_some() {
                break;
            }
        }

        assert!(universe.success_state.is_some());
    }
}