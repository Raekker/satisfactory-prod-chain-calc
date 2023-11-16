use std::collections::HashMap;

mod prodcalc;

use prodcalc::{Machine, MachineName, ProdItem, ProductionCalculator, Recipe};

fn main() {
    let recipes: HashMap<ProdItem, Recipe> = HashMap::from([
        (
            ProdItem::IronIngot,
            Recipe::new((ProdItem::IronIngot, 30.0), vec![(ProdItem::IronOre, 30.0)]),
        ),
        (
            ProdItem::IronRod,
            Recipe::new((ProdItem::IronRod, 15.0), vec![(ProdItem::IronIngot, 15.0)]),
        ),
        (
            ProdItem::IronOre,
            Recipe::new((ProdItem::IronOre, 60.0), vec![(ProdItem::IronOre, 0.0)]),
        ),
        (
            ProdItem::Screw,
            Recipe::new((ProdItem::Screw, 40.0), vec![(ProdItem::IronRod, 10.0)]),
        ),
        (
            ProdItem::IronPlate,
            Recipe::new(
                (ProdItem::IronPlate, 20.0),
                vec![(ProdItem::IronIngot, 30.0)],
            ),
        ),
        (
            ProdItem::ReinforcedIronPlate,
            Recipe::new(
                (ProdItem::ReinforcedIronPlate, 5.0),
                vec![(ProdItem::IronPlate, 30.0), (ProdItem::Screw, 60.0)],
            ),
        ),
        (
            ProdItem::SteelIngot,
            Recipe::new(
                (ProdItem::SteelIngot, 45.0),
                vec![(ProdItem::IronOre, 45.0), (ProdItem::Coal, 45.0)],
            ),
        ),
        (
            ProdItem::AluminiumIngot,
            Recipe::new(
                (ProdItem::AluminiumIngot, 60.0),
                vec![(ProdItem::AluminiumScrap, 90.0), (ProdItem::Sillica, 75.0)],
            ),
        ),
        (
            ProdItem::CopperIngot,
            Recipe::new(
                (ProdItem::CopperIngot, 30.0),
                vec![(ProdItem::CopperOre, 30.0)],
            ),
        ),
        (
            ProdItem::Wire,
            Recipe::new((ProdItem::Wire, 30.0), vec![(ProdItem::CopperIngot, 15.0)]),
        ),
        (
            ProdItem::Cable,
            Recipe::new((ProdItem::Cable, 30.0), vec![(ProdItem::Wire, 60.0)]),
        ),
        (
            ProdItem::CopperSheet,
            Recipe::new(
                (ProdItem::CopperSheet, 10.0),
                vec![(ProdItem::CopperIngot, 20.0)],
            ),
        ),
    ]);
    let machinexitems: HashMap<MachineName, Vec<ProdItem>> = HashMap::from([
        (
            MachineName::Smelter,
            vec![ProdItem::IronIngot, ProdItem::CopperIngot],
        ),
        (
            MachineName::Constructor,
            vec![
                ProdItem::IronRod,
                ProdItem::Screw,
                ProdItem::IronPlate,
                ProdItem::Wire,
                ProdItem::Cable,
                ProdItem::CopperSheet,
            ],
        ),
        (
            MachineName::MinerMk1,
            vec![
                ProdItem::IronOre,
                ProdItem::CopperOre,
                ProdItem::QuartzOre,
                ProdItem::Coal,
                ProdItem::LimeStone,
            ],
        ),
        (MachineName::Assembler, vec![ProdItem::ReinforcedIronPlate]),
        (
            MachineName::Foundry,
            vec![ProdItem::SteelIngot, ProdItem::AluminiumIngot],
        ),
    ]);

    let machines = HashMap::from([
        (
            MachineName::Smelter,
            Machine::new(MachineName::Smelter, 4.0),
        ),
        (
            MachineName::Constructor,
            Machine::new(MachineName::Constructor, 4.0),
        ),
        (
            MachineName::MinerMk1,
            Machine::new(MachineName::MinerMk1, 5.0),
        ),
        (
            MachineName::Assembler,
            Machine::new(MachineName::Assembler, 16.0),
        ),
        (
            MachineName::Foundry,
            Machine::new(MachineName::Foundry, 16.0),
        ),
    ]);

    let production_calculator = ProductionCalculator::new(recipes, machines, machinexitems);
    production_calculator.run();
}
