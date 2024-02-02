
#[derive(Debug)]
pub struct Traits {
    pub nrg: i32,
    pub agg: i32,
    pub spk: i32,
    pub brn: i32,
    pub eyc: i32,
    pub eys: i32
}

#[derive(Debug, PartialEq, Clone)]
pub enum Trait {
    NRG,
    AGG,
    SPK,
    BRN,
    EYC,
    EYS,
    BRS
}

#[derive(Debug, PartialEq, Clone)]
pub enum Class {
    Ninja,
    Enlightened,
    Cleaver,
    Tank,
    Cursed,
    Healer,
    Mage,
    Troll
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stats {
    pub speed: u32,
    pub health: u32,
    pub crit: u32,
    pub armor: u32,
    pub evade: u32,
    pub resist: u32,
    pub magic_power: u32,
    pub physical_power: u32,
    pub accuracy: u32
}

#[derive(Debug, PartialEq, Clone)]
pub struct Gotchi {
    pub class: Class,
    pub stats: Stats,
    pub brs: f64
}

impl Gotchi {
    pub fn new(traits: &[i32; 7]) -> Self {
        let brs = traits[6] as f64;

        let traits = Traits {
            nrg: traits[0],
            agg: traits[1],
            spk: traits[2],
            brn: traits[3],
            eyc: traits[4],
            eys: traits[5],
        };

        let stats = Self::calc_stats(&traits, brs);
        let class = Self::get_class(&traits);

        Gotchi {
            class,
            stats,
            brs
        }
    }

    fn get_class(traits: &Traits) -> Class {
        let traits: [i32; 4] = [traits.nrg, traits.agg, traits.spk, traits.brn];
        let (max_index, _) = traits.iter().enumerate().max_by_key(|(_, &x)| x).unwrap();
        let (min_index, _) = traits.iter().enumerate().min_by_key(|(_, &x)| x).unwrap();

        // TODO this doesnt allow for multiple highs / lows to be the same
        let best_high = 100 - traits[max_index];
        let best_low = traits[min_index];

        if best_high < best_low {
            match max_index {
                0 => Class::Ninja,
                1 => Class::Cleaver,
                2 => Class::Cursed,
                3 => Class::Mage,
                _ => { panic!("Not a valid class. Invalid max index") }
            }
            // TODO handle if best_high === best_low
        } else {
            match min_index {
                0 => Class::Enlightened,
                1 => Class::Tank,
                2 => Class::Healer,
                3 => Class::Troll,
                _ => { panic!("Not a valid class. Invalid max index") }
            }
        }
    }

    fn calc_stats_pair_from_trait(
        r_trait: i32,
        base_1: f64,
        base_2: f64,
        multiplier_1: f64,
        multiplier_2: f64
    ) -> (u32, u32) {
        if r_trait >= 50 {
            (
                (base_1 + ((r_trait - 50 + 1) as f64 * multiplier_1)).ceil() as u32,
                base_2 as u32,
            )
        } else {
            (
                base_1 as u32,
                (base_2 + ((50.0 - r_trait as f64) * multiplier_2)).ceil() as u32
            )
        }
    }

    fn calc_stats(traits: &Traits, brs: f64) -> Stats {

        // Stat Base Value + (Trait Value * Stat Multiplier)
        let (speed, health) = {
            let speed_base = 100.0;
            let health_base = (brs * 0.75).ceil();

            Self::calc_stats_pair_from_trait(traits.nrg, speed_base, health_base, 1.0, 10.0)
        };

        let (crit, armor) = Self::calc_stats_pair_from_trait(traits.agg, 0.0, 0.0, 0.5, 2.0);
        let (evade, resist) = Self::calc_stats_pair_from_trait(traits.spk, 0.0, 0.0, 0.5, 1.5);
        let (magic_power, physical_power) = {
            let base = (brs * 0.25).ceil();
            Self::calc_stats_pair_from_trait(traits.brn, base, base, 5.0, 5.0)
        };

        let accuracy = {
            let base_accuracy = 50;
            let (eyc_1, eyc_2) = Self::calc_stats_pair_from_trait(traits.eyc, 0.0, 0.0, 0.5, 0.5);
            let (eys_1, eys_2) = Self::calc_stats_pair_from_trait(traits.eys, 0.0, 0.0, 0.5, 0.5);

            base_accuracy + std::cmp::max(eyc_1, eyc_2) + std::cmp::max(eys_1, eys_2)
        };

        Stats {
            speed,
            health,
            crit,
            armor,
            evade,
            resist,
            magic_power,
            physical_power,
            accuracy
        }
    }
}



// TODO load gotchi
    // let load_gotchi = {
        // let ips = ips.clone();
        // Callback::from(move |_| {
            // let ips = ips.clone();
            // spawn_local(async move {
                // let fetched_ip: Ip = Request::get("http://httpbin.org/ip")
                    // .send()
                    // .await
                    // .unwrap()
                    // .json()
                    // .await
                    // .unwrap();

                // log!(format!("Ip: {:?}", fetched_ip));
                // ips.set(Some(fetched_ip));
            // });
        // })
    // };
