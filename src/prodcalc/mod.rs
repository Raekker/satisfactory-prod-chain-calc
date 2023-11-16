use std::collections::HashMap;
use std::hash::Hash;
use std::io::stdin;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ProdItem {
    IronIngot,
    CopperIngot,
    IronRod,
    IronOre,
    CopperOre,
    Screw,
    IronPlate,
    ReinforcedIronPlate,
    AluminiumIngot,
    Wire,
    Cable,
    CopperSheet,
    SteelIngot,
    Coal,
    LimeStone,
    QuartzOre,
    AluminiumScrap,
    Sillica,
}

impl ProdItem {
    fn from_string(input_str: &str) -> Option<Self> {
        use ProdItem::*;
        match input_str {
            "IronIngot" => Some(IronIngot),
            "CopperIngot" => Some(CopperIngot),
            "IronRod" => Some(IronRod),
            "IronOre" => Some(IronOre),
            "CopperOre" => Some(CopperOre),
            "Screw" => Some(Screw),
            "IronPlate" => Some(IronPlate),
            "ReinforcedIronPlate" => Some(ReinforcedIronPlate),
            "AluminiumIngot" => Some(AluminiumIngot),
            "Wire" => Some(Wire),
            "Cable" => Some(Cable),
            "CopperSheet" => Some(CopperSheet),
            "SteelIngot" => Some(SteelIngot),
            "Coal" => Some(Coal),
            "LimeStone" => Some(LimeStone),
            "QuartzOre" => Some(QuartzOre),
            "AluminiumScrap" => Some(AluminiumScrap),
            "Sillica" => Some(Sillica),
            _ => None,
        }
    }

    fn is_raw_resource(&self) -> bool {
        use ProdItem::*;
        match self {
            IronOre | CopperOre | Coal | LimeStone | QuartzOre => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Recipe {
    output_item: (ProdItem, f64),
    input_items: Vec<(ProdItem, f64)>,
}

impl Recipe {
    pub fn new(output_item: (ProdItem, f64), input_items: Vec<(ProdItem, f64)>) -> Self {
        Self {
            output_item,
            input_items,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MachineName {
    Smelter,
    Constructor,
    MinerMk1,
    Assembler,
    Foundry,
}

impl MachineName {
    fn from_string(input_str: &str) -> Option<Self> {
        use MachineName::*;
        match input_str {
            "Smelter" => Some(Smelter),
            "Constructor" => Some(Constructor),
            "Assembler" => Some(Assembler),
            "MinerMk1" => Some(MinerMk1),
            "Foundry" => Some(Foundry),
            _ => None,
        }
    }
}

pub struct Machine {
    name: MachineName,
    power_consumption: f64,
}

impl Machine {
    pub fn new(name: MachineName, power_consumption: f64) -> Self {
        Self {
            name,
            power_consumption,
        }
    }
}

pub struct ProductionCalculator {
    recipes: HashMap<ProdItem, Recipe>,
    machinexitems: HashMap<MachineName, Vec<ProdItem>>,
    machines: HashMap<MachineName, Machine>,
}

impl ProductionCalculator {
    pub fn new(
        recipes: HashMap<ProdItem, Recipe>,
        machines: HashMap<MachineName, Machine>,
        machinexitems: HashMap<MachineName, Vec<ProdItem>>,
    ) -> Self {
        Self {
            recipes,
            machines,
            machinexitems,
        }
    }

    pub fn run(&self) {
        let mut user_input = String::new();
        println!(r#"If you wish to exit type in "q""#);
        loop {
            user_input.clear();
            println!("Input desired item: ");
            stdin().read_line(&mut user_input).expect("Incorrect input");
            if user_input.trim() == "q" {
                println!("Goodbye!");
                break;
            }
            let user_item: ProdItem;
            match ProdItem::from_string(user_input.trim()) {
                Some(item) => user_item = item,
                None => {
                    println!("Invalid item try again");
                    continue;
                }
            };
            user_input.clear();
            println!("Input desired quantity: ");
            stdin().read_line(&mut user_input).expect("Incorrect input");
            let qty_per_min: f64;
            match user_input.trim().parse::<f64>() {
                Ok(value) => qty_per_min = value,
                Err(err) => {
                    println!("Error {}", err);
                    continue;
                }
            }
            let mut needed_resources: HashMap<&ProdItem, (&MachineName, u32, f64)> = HashMap::new();
            let mut power_consumption = 0.0;
            let user_recipe: &Recipe = self.recipes.get(&user_item).unwrap();

            let user_machine = self
                .machinexitems
                .iter()
                .find_map(|x| {
                    let (key, value) = x;
                    if value.contains(&user_item) {
                        Some(key)
                    } else {
                        None
                    }
                })
                .unwrap();

            let needed_machines = (qty_per_min / user_recipe.output_item.1).ceil();
            power_consumption +=
                needed_machines * self.machines.get(&user_machine).unwrap().power_consumption;

            needed_resources.insert(
                &user_recipe.output_item.0,
                (
                    user_machine,
                    needed_machines as u32,
                    needed_machines * user_recipe.output_item.1,
                ),
            );
            // .entry(&user_recipe.output_item.0)
            // .or_insert((
            //     user_machine,
            //     needed_machines as u32,
            //     needed_machines * user_recipe.output_item.1,
            // ));
            let mut item_queue: Vec<(&ProdItem, &f64, u32)> = user_recipe
                .input_items
                .iter()
                .map(|(item, per_min)| (item, per_min, needed_machines as u32))
                .collect();

            loop {
                if item_queue.is_empty() {
                    break;
                }
                for (item, amount_per_min, needed_machines) in item_queue.clone() {
                    // println!("{item:?}");
                    let machine = self
                        .machinexitems
                        .iter()
                        .find_map(|x| {
                            let (key, value) = x;
                            if value.contains(&item) {
                                Some(key)
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    let item_recipe: &Recipe = self.recipes.get(&item).unwrap();
                    let item_machines = (needed_machines as f64 * *amount_per_min
                        / item_recipe.output_item.1)
                        .ceil();
                    power_consumption += needed_machines as f64
                        * self.machines.get(&machine).unwrap().power_consumption;
                    needed_resources
                        .entry(&item)
                        .and_modify(|(_, im, nm)| {
                            *im += item_machines as u32;
                            *nm += needed_machines as f64 * *amount_per_min
                        })
                        .or_insert((
                            machine,
                            item_machines as u32,
                            needed_machines as f64 * *amount_per_min,
                        ));

                    item_queue.remove(0);
                    if item.is_raw_resource() {
                        continue;
                    }

                    item_queue.extend(
                        item_recipe
                            .input_items
                            .iter()
                            .map(|(item, per_min)| (item, per_min, item_machines as u32)),
                    );

                    // println!("{machine:?}");
                }
            }

            println!("Needed resources: {needed_resources:?}\nPower consuption in {power_consumption} MW");
            // println!("Your recipe: {user_recipe:?} and machine {needed_machines:?}");
        }
    }
}
