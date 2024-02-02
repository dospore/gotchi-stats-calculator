use yew::prelude::*;
use web_sys::{Event, HtmlInputElement};
use web_sys::wasm_bindgen::JsCast;
use crate::gotchi::{Gotchi, Trait};

#[derive(Debug, Properties, PartialEq )]
struct GotchiDisplayProps {
    gotchi: Option<Gotchi>,
}

#[function_component(GotchiDisplay)]
fn gotchi_display(GotchiDisplayProps { gotchi }: &GotchiDisplayProps) -> Html {
    if let Some(gotchi) = gotchi {
        html! {
            <div>
                <div>{format!("Class: {:?}", gotchi.class)}</div>
                <div>{format!("BRS: {:?}", gotchi.brs)}</div>
                <div>{format!("Speed: {:?}", gotchi.stats.speed)}</div>
                <div>{format!("Health: {:?}", gotchi.stats.health)}</div>
                <div>{format!("Crit Chance: {:?}%", gotchi.stats.crit)}</div>
                <div>{format!("Armor: {:?}", gotchi.stats.armor)}</div>
                <div>{format!("Evade Chance: {:?}%", gotchi.stats.evade)}</div>
                <div>{format!("Resist Chance: {:?}%", gotchi.stats.resist)}</div>
                <div>{format!("Magic power: {:?}", gotchi.stats.magic_power)}</div>
                <div>{format!("Physical power: {:?}", gotchi.stats.physical_power)}</div>
                <div>{format!("Accuracy: {:?}", gotchi.stats.accuracy)}</div>
            </div>
        }
    } else {
        html! {
            <div>
                {" No Gotchi Stats "}
            </div>
        }
    }
}

#[derive(Debug, Properties, PartialEq )]
struct TraitControlProps {
    t: Trait,
    v: i32,
    handle_click: Callback<(Trait, bool, bool)>,
}

#[function_component(TraitControl)]
fn trait_control(TraitControlProps { t, v, handle_click }: &TraitControlProps) -> Html {
    html! {
        <div class="trait-controls">
            <button onclick={
                let handle_click = handle_click.clone();
                let t = t.clone();
                Callback::from(move |_| {
                    handle_click.emit((t.clone(), false, true))
                })
            }>{" << "}</button>
            <button onclick={
                let handle_click = handle_click.clone();
                let t = t.clone();
                Callback::from(move |_| {
                    handle_click.emit((t.clone(), false, false))
                })
            }>{" < "}</button>
            <span>{format!("{:?} : {}", t, v)}</span>
            <button onclick={
                let handle_click = handle_click.clone();
                let t = t.clone();
                Callback::from(move |_| {
                    handle_click.emit((t.clone(), true, false))
                })
            }>{" > "}</button>
            <button onclick={
                let handle_click = handle_click.clone();
                let t = t.clone();
                Callback::from(move |_| {
                    handle_click.emit((t.clone(), true, true))
                })
            }>{" >> "}</button>
        </div>
    }
}

#[function_component(GithubLogo)]
pub fn github_logo() -> Html {
    html! {
        <svg height="100%" width="100%" viewBox="0 0 78 96"  xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z" fill="currentColor"/></svg>
    }

}


#[function_component(App)]
pub fn app() -> Html {

    let starting_gotchi_traits = [50, 50, 50, 50, 50, 50, 500];
    let gotchi = use_state(|| Gotchi::new(&starting_gotchi_traits));
    let traits = use_state(|| starting_gotchi_traits);

    let gotchi_id = use_state(|| "".to_owned());

    let onchange = {
        let gotchi_id = gotchi_id.clone();

        move |event: Event| {
            let v = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            gotchi_id.set(v);
        }
    };

    // let load_gotchi = {
        // let gotchi = gotchi.clone();
        // Callback::from(move |_| {
            // let g = Gotchi::new(&[100, 100, 100, 100], 500.0);
            // gotchi.set(Some(g));
        // })
    // };

    let handle_click = {
        let traits = traits.clone();
        let gotchi = gotchi.clone();
        Callback::from(move |(t, increment, big)| {
            let mut current = *traits;
            let i = match t {
                Trait::NRG => 0,
                Trait::AGG => 1,
                Trait::SPK => 2,
                Trait::BRN => 3,
                Trait::EYC => 4,
                Trait::EYS => 5,
                Trait::BRS => 6
            };
            let size = if big { 10 } else { 1 };
            if increment {
                current[i] += size;
            } else {
                current[i] -= size;
            }
            traits.set(current);
            gotchi.set(Gotchi::new(&current));
        })
    };

    html! {
        <main>
            <h1>{ "A very basic gotchi battler stats calculator" }</h1>
            // <input {onchange} placeholder="Enter gotchi id" />
            // <button onclick={load_gotchi}>{" Load gotchi "}</button>
            // <span>{format!("Gotchi id: {}", *gotchi_id)}</span>
            <div class="container">
                <div class="controls">
                    <TraitControl 
                        handle_click={handle_click.clone()}
                        t={Trait::BRS}
                        v={(*traits)[6] as i32}
                    />
                    <TraitControl 
                        handle_click={handle_click.clone()}
                        t={Trait::NRG}
                        v={(*traits)[0] as i32}
                    />
                    <TraitControl 
                        handle_click={handle_click.clone()}
                        t={Trait::AGG}
                        v={(*traits)[1] as i32}
                    />
                    <TraitControl 
                        handle_click={handle_click.clone()}
                        t={Trait::SPK}
                        v={(*traits)[2] as i32}
                    />
                    <TraitControl 
                        handle_click={handle_click.clone()}
                        t={Trait::BRN}
                        v={(*traits)[3] as i32}
                    />
                    <TraitControl 
                        handle_click={handle_click.clone()}
                        t={Trait::EYC}
                        v={(*traits)[4] as i32}
                    />
                    <TraitControl 
                        handle_click={handle_click.clone()}
                        t={Trait::EYS}
                        v={(*traits)[5] as i32}
                    />
                </div>
                <div class="display">
                    <GotchiDisplay gotchi={(*gotchi).clone()} />
                </div>
            </div>
            <div class="footer">
                {"Built from the "}
                <a href={"https://gotchi-battler-1.gitbook.io/gotchi-battler/battle-mechanics/trait-mapping"}>{"documentation"}</a><br />
                {" as a pracitce project of building applications with "}
                <a href={"https://yew.rs/"}>{"rust + yew."}</a>
                <div class="logo">
                    <GithubLogo />
                </div>
            </div>
        </main>
    }
}
