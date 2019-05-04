/* # [[[cog

import re
from pathlib import Path

types_file = Path(r'D:\git\rgb_delivery\rgb-solver\pkg\rgb_solver.d.ts')

#Path(fname).parent / ".." / ".." / "rgb-solver" / "pkg" / "rgb_solver.d"
#D:\git\rgb_delivery\web_worker\worker\index.ts
#

def remove_double_newlines(s):
    #return s
    return re.sub(r'\n', ' \n', s)

type_list = []

for line in open(types_file, "r"):

    m = re.search("export type (\w+)", line, re.IGNORECASE)

    if m:
        type_list.append(m.group(1))

type_list = ",\n".join(type_list)

cog.out(f"""
export {{ {type_list} }} from "rgb-solver";
""")
#]]] */

export { Color,
ColorIndex,
CellIndex,
VanIndex,
Button,
NavigableTileStatic,
NavigableTileDynamic,
Road,
Bridge,
Warehouse,
TileEnum,
TileRoad,
TileWarehouse,
TileBridge,
Empty,
TileEnum_type,
CellData,
ChoiceOverride,
CalculationResponse,
Van,
GridState } from "rgb-solver";
// --[[[end]]]

/*export {CellData, ChoiceOverride, GridState, TileEnum_type,
    TileEnum, ColorIndex, Color, Button, TileRoad} from "rgb-solver";*/
export * from "./interface_types";
