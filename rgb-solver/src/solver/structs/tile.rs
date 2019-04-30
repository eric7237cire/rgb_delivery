#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum TileEnum {
    TileRoad(Road),
    TileWarehouse(Warehouse),
    TileBridge(Bridge),
    Empty,
}

use TileEnum::*;

impl TileEnum {

    pub fn get_van(&self) -> Option<&Van> {
        match self {
            TileRoad(Road { dynamic_attrs, .. }) | TileBridge(Bridge { dynamic_attrs, .. }) => dynamic_attrs.van_snapshot.as_ref(),
            Empty | TileWarehouse(..) => None
        }
    }

    pub fn set_van(&mut self, van: &Van) {
        match self {
            TileRoad(Road { dynamic_attrs, .. }) | TileBridge(Bridge { dynamic_attrs, .. }) => dynamic_attrs.van_snapshot = Some(van.clone()),
            Empty | TileWarehouse(..) => {}
        }
    }

    pub fn get_connection_mask(&self) -> Option<u8> {
        match self {
            TileRoad(Road { static_attrs, .. }) | TileBridge(Bridge { static_attrs, .. }) => Some(static_attrs.connection_mask),
            //=> Some(static_attrs.connection_mask),
            Empty | TileWarehouse(..) => None
        }
    }

    pub fn set_leaving_van(&mut self, van_index: VanIndex, tick: usize, dir_index: usize) {
        match self
            {
                TileRoad(Road { dynamic_attrs, used_tick, used_van_index, .. }) => {
                    dynamic_attrs.van_snapshot = None;
                    used_tick[dir_index] = Some(tick);
                    used_van_index[dir_index] = Some(van_index);
                }
                TileBridge(Bridge { dynamic_attrs, used_van_index, used_tick, is_up, .. }) => {
                    assert!(!*is_up);

                    //These are set when moved to
                    assert_eq!(*used_van_index, Some(van_index));
                    assert_eq!(*used_tick, Some(tick - 1));

                    dynamic_attrs.van_snapshot = None;
                    *used_tick = Some(tick);
                    *used_van_index = Some(van_index);
                }
                Empty | TileWarehouse(..) => panic!(),
            }
    }

    pub fn set_arriving_van(&mut self, van_index: VanIndex, van: &Van, tick: usize, dir_index: usize) {
        match self {
            TileRoad(next_road) =>
                {
                    //keep a history
                    next_road.dynamic_attrs.van_snapshot = Some(van.clone());

                    next_road.used_van_index[dir_index] = Some(van_index);
                    next_road.used_tick[dir_index] = Some(tick);
                }
            TileBridge(next_bridge) =>
                {
                    assert!(next_bridge.dynamic_attrs.van_snapshot.is_none());
                    assert!(next_bridge.used_van_index.is_none());
                    assert!(next_bridge.used_tick.is_none());

                    next_bridge.dynamic_attrs.van_snapshot = Some(van.clone());

                    next_bridge.used_van_index = Some(van_index);

                    next_bridge.used_tick = Some(tick);
                }
            Empty | TileWarehouse(..) => panic!("not a road or bridge")
        }
    }

    pub fn reset(&mut self) {
        match self {
                TileRoad(road) => {
                    road.used_van_index = Default::default();
                    road.dynamic_attrs.van_snapshot = None;
                }
                TileBridge(bridge) => {
                    bridge.used_tick = None;
                    bridge.used_van_index = None;
                    bridge.dynamic_attrs.van_snapshot = None;
                }
                Empty | TileWarehouse(..) => {}
            }
    }
}