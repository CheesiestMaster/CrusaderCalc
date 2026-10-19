use wasm_bindgen::prelude::*;

use web_sys::{HtmlButtonElement, HtmlFormElement, HtmlImageElement, HtmlInputElement, HtmlOptionElement, HtmlSelectElement, HtmlParagraphElement};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

use data::{FormState, ValidFor, ValidForArray, UnitType, Asset, Field, FieldArray};

#[cfg(debug_assertions)]
macro_rules! debug {
    ($($t:tt)*) => {
        web_sys::console::debug_1(&format!($($t)*).into())
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($($t:tt)*) => {};
}


macro_rules! info {
    ($($t:tt)*) => (web_sys::console::info_1(&format!($($t)*).into()))
}

macro_rules! warn {
    ($($t:tt)*) => (web_sys::console::warn_1(&format!($($t)*).into()))
}

macro_rules! error {
    ($($t:tt)*) => (web_sys::console::error_1(&format!($($t)*).into()))
}

mod data;

macro_rules! break_line {
    ($d:ident, $parent:ident) => {
        $parent.append_child(&$d.create_element("br").unwrap()).unwrap();
    }
}

macro_rules! horizontal_rule {
    ($d:ident, $parent:ident) => {
        $parent.append_child(&$d.create_element("hr").unwrap()).unwrap();
    }
}

macro_rules! get_element_by_id {
    ($element_id:expr) => {
        web_sys::window().unwrap().document().unwrap().get_element_by_id($element_id).unwrap()
    }
}

macro_rules! log_select {
    ($select:expr, $value:expr) => {{
        let value = $value;
        $select.set_value(&value);
        let index = $select.selected_index();
        debug!("{} value {}, selected_index {}", stringify!($select), value, index);
        if index < 0 {
            error!("{} value {} is not an option", stringify!($select), value);
        }
    }};
}

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    let d = web_sys::window().unwrap().document().unwrap();
    debug!("wasm start, ready_state {}", d.ready_state());
    if d.ready_state() == "loading" {
        info!("waiting for DOMContentLoaded");
        let _ = d.add_event_listener_with_callback("DOMContentLoaded", &Closure::once_into_js(run).unchecked_into());
    }
    else {
        info!("document already loaded");
        run();
    }
}

fn run() {
    let d = web_sys::window().unwrap().document().unwrap(); 
    let body = d.body().unwrap();
    let local_storage = match web_sys::window().unwrap().local_storage() {
        Ok(storage) => storage,
        Err(err) => {
            error!("local storage lookup failed: {:?}", err);
            None
        }
    };
    let search = match web_sys::window().unwrap().location().search() {
        Ok(search) => search,
        Err(err) => {
            error!("failed to read location search: {:?}", err);
            String::new()
        }
    };
    let params = match web_sys::UrlSearchParams::new_with_str(&search) {
        Ok(params) => params,
        Err(err) => {
            error!("failed to parse location search {:?}: {:?}", search, err);
            web_sys::UrlSearchParams::new().unwrap()
        }
    };
    let encoded = params.get("L").unwrap_or_default();
    info!("link param L: {:?}", encoded);
    let link = if encoded.is_empty() {
        debug!("no link param, using the default form");
        Vec::new()
    } else {
        match URL_SAFE_NO_PAD.decode(&encoded) {
            Ok(bytes) if bytes.len() == 35 && bytes.first() == Some(&1) => {
                info!("loaded form from link, {} bytes", bytes.len());
                bytes
            }
            Ok(bytes) => {
                warn!("ignoring link payload version/len {:?}/{}, expected 1/35", bytes.first(), bytes.len());
                Vec::new()
            }
            Err(err) => {
                error!("failed to decode link param: {}", err);
                Vec::new()
            }
        }
    };
    let mut form_state: FormState = link.as_slice().into();
    let h1 = d.create_element("h1").unwrap();
    h1.set_text_content(Some("Castles and Crusaders Unit Builder"));
    body.append_child(&h1).unwrap();
    debug!("header created");
    let form = d.create_element("form").unwrap().unchecked_into::<HtmlFormElement>();
    form.set_id("form");
    body.append_child(&form).unwrap();
    debug!("form created");
    let gold_budget_label = d.create_element("label").unwrap();
    gold_budget_label.set_text_content(Some("Gold Budget "));
    form.append_child(&gold_budget_label).unwrap();
    debug!("gold budget label created");
    let gold_budget_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    gold_budget_input.set_id("gold_budget");
    gold_budget_input.set_type("number");
    gold_budget_input.set_min("0");
    gold_budget_input.set_max("65535");
    gold_budget_input.set_name("budget");
    gold_budget_input.set_value(form_state.budget.to_string().as_str());
    form.append_child(&gold_budget_input).unwrap();
    debug!("gold budget input created");
    break_line!(d, form);
    debug!("break line created");
    let remaining_budget_label = d.create_element("label").unwrap();
    remaining_budget_label.set_text_content(Some("Remaining Budget "));
    form.append_child(&remaining_budget_label).unwrap();
    debug!("remaining budget label created");
    let remaining_budget_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    remaining_budget_input.set_id("remaining_budget");
    remaining_budget_input.set_type("number");
    remaining_budget_input.set_placeholder("0");
    remaining_budget_input.set_read_only(true);
    form.append_child(&remaining_budget_input).unwrap();
    debug!("remaining budget input created");
    break_line!(d, form);
    debug!("break line created");
    let unit_cost_label = d.create_element("label").unwrap();
    unit_cost_label.set_text_content(Some("Unit Cost "));
    form.append_child(&unit_cost_label).unwrap();
    debug!("unit cost label created");
    let unit_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    unit_cost_input.set_id("unit_cost");
    unit_cost_input.set_type("number");
    unit_cost_input.set_placeholder("0");
    unit_cost_input.set_read_only(true);
    form.append_child(&unit_cost_input).unwrap();
    debug!("unit cost input created");
    break_line!(d, form);
    debug!("break line created");
    let unit_gold_remaining_label = d.create_element("label").unwrap();
    unit_gold_remaining_label.set_text_content(Some("Unit Gold Remaining "));
    form.append_child(&unit_gold_remaining_label).unwrap();
    debug!("unit gold remaining label created");
    let unit_gold_remaining_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    unit_gold_remaining_input.set_id("unit_gold_remaining");
    unit_gold_remaining_input.set_type("number");
    unit_gold_remaining_input.set_placeholder("0");
    unit_gold_remaining_input.set_read_only(true);
    form.append_child(&unit_gold_remaining_input).unwrap();
    debug!("unit gold remaining input created");
    horizontal_rule!(d, form);
    debug!("horizontal rule created");
    let unit_type_label = d.create_element("label").unwrap();
    unit_type_label.set_text_content(Some("Unit Type "));
    form.append_child(&unit_type_label).unwrap();
    debug!("unit type label created");
    let unit_type_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    unit_type_select.set_id("unit_type");
    unit_type_select.set_name("unit_type");
    let valid_unit_types = UnitType::valid_for(&form_state);
    valid_unit_types.iter().map(|&ut| Into::<HtmlOptionElement>::into(ut)).for_each(|option| {unit_type_select.append_child(&option).unwrap();});
    form_state.unit_type = UnitType::resolve_or_default(&form_state, valid_unit_types.as_slice());
    log_select!(unit_type_select, Into::<u8>::into(form_state.unit_type).to_string());
    form.append_child(&unit_type_select).unwrap();
    debug!("unit type select created");
    let unit_type_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    unit_type_image.set_id("unit_type_image");
    unit_type_image.set_src(form_state.unit_type.asset().as_str());
    form.append_child(&unit_type_image).unwrap();
    debug!("unit type image created");
    let refit_type_label = d.create_element("label").unwrap();
    refit_type_label.set_text_content(Some("Refit Type "));
    form.append_child(&refit_type_label).unwrap();
    debug!("refit type label created");
    let refit_type_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    refit_type_select.set_id("refit_type");
    refit_type_select.set_name("refit_type");
    let valid_refit_types = data::RefitType::valid_for(&form_state);
    valid_refit_types.iter().map(|&rt| Into::<HtmlOptionElement>::into(rt)).for_each(|option| {refit_type_select.append_child(&option).unwrap();});
    form_state.refit_type = data::RefitType::resolve_or_default(&form_state, valid_refit_types.as_slice());
    log_select!(refit_type_select, Into::<u8>::into(form_state.refit_type).to_string());
    form.append_child(&refit_type_select).unwrap();
    debug!("refit type select created");
    let refit_type_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    refit_type_image.set_id("refit_type_image");
    refit_type_image.set_src(form_state.refit_type.asset().as_str());
    form.append_child(&refit_type_image).unwrap();
    debug!("refit type image created");
    break_line!(d, form);
    let unit_gold_label = d.create_element("label").unwrap();
    unit_gold_label.set_text_content(Some("Unit Gold "));
    form.append_child(&unit_gold_label).unwrap();
    debug!("unit gold label created");
    let unit_gold_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    unit_gold_input.set_id("unit_gold");
    unit_gold_input.set_type("number");
    unit_gold_input.set_value(&form_state.unit_type.unit_gold().to_string());
    unit_gold_input.set_read_only(true);
    form.append_child(&unit_gold_input).unwrap();
    debug!("unit gold input created");
    let refit_gold_label = d.create_element("label").unwrap();
    refit_gold_label.set_text_content(Some("Refit Gold "));
    form.append_child(&refit_gold_label).unwrap();
    debug!("refit gold label created");
    let refit_gold_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    refit_gold_input.set_id("refit_gold");
    refit_gold_input.set_type("number");
    refit_gold_input.set_value(&form_state.refit_type.unit_gold().to_string());
    refit_gold_input.set_read_only(true);
    form.append_child(&refit_gold_input).unwrap();
    debug!("refit gold input created");
    break_line!(d, form);
    debug!("break line created");
    let siege_label = d.create_element("label").unwrap();
    siege_label.set_text_content(Some("Siege "));
    form.append_child(&siege_label).unwrap();
    debug!("siege label created");
    let siege_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    siege_select.set_id("siege");
    siege_select.set_name("siege");
    let valid_sieges = data::Siege::valid_for(&form_state);
    valid_sieges.iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {siege_select.append_child(&option).unwrap();});
    form_state.siege = data::Siege::resolve_or_default(&form_state, valid_sieges.as_slice());
    log_select!(siege_select, Into::<u8>::into(form_state.siege).to_string());
    form.append_child(&siege_select).unwrap();
    debug!("siege select created");
    let siege_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    siege_image.set_id("siege_image");
    siege_image.set_src(form_state.siege.asset().as_str());
    form.append_child(&siege_image).unwrap();
    debug!("siege image created");
    let heavy_siege_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    heavy_siege_select.set_id("heavy_siege");
    heavy_siege_select.set_name("heavy_siege");
    let valid_heavy_sieges = data::HeavySiege::valid_for(&form_state);
    valid_heavy_sieges.iter().map(|&hs| Into::<HtmlOptionElement>::into(hs)).for_each(|option| {heavy_siege_select.append_child(&option).unwrap();});
    form_state.heavy_siege = data::HeavySiege::resolve_or_default(&form_state, valid_heavy_sieges.as_slice());
    log_select!(heavy_siege_select, Into::<u8>::into(form_state.heavy_siege).to_string());
    form.append_child(&heavy_siege_select).unwrap();
    debug!("heavy siege select created");
    let heavy_siege_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    heavy_siege_image.set_id("heavy_siege_image");
    heavy_siege_image.set_src(form_state.heavy_siege.asset().as_str());
    form.append_child(&heavy_siege_image).unwrap();
    debug!("heavy siege image created");
    let pack_label = d.create_element("label").unwrap();
    pack_label.set_text_content(Some("Pack "));
    form.append_child(&pack_label).unwrap();
    debug!("pack label created");
    let pack_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    pack_select.set_id("pack");
    pack_select.set_name("pack");
    let valid_packs = data::Pack::valid_for(&form_state);
    valid_packs.iter().map(|&p| Into::<HtmlOptionElement>::into(p)).for_each(|option| {pack_select.append_child(&option).unwrap();});
    form_state.pack = data::Pack::resolve_or_default(&form_state, valid_packs.as_slice());
    log_select!(pack_select, Into::<u8>::into(form_state.pack).to_string());
    form.append_child(&pack_select).unwrap();
    debug!("pack select created");
    let pack_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    pack_image.set_id("pack_image");
    pack_image.set_src(form_state.pack.asset().as_str());
    form.append_child(&pack_image).unwrap();
    debug!("pack image created");
    let potion_label = d.create_element("label").unwrap();
    potion_label.set_text_content(Some("Potion "));
    form.append_child(&potion_label).unwrap();
    debug!("potion label created");
    let potion_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    potion_select.set_id("potion");
    potion_select.set_name("potion");
    let valid_potions = data::Potion::valid_for(&form_state);
    valid_potions.iter().map(|&p| Into::<HtmlOptionElement>::into(p)).for_each(|option| {potion_select.append_child(&option).unwrap();});
    form_state.potion = data::Potion::resolve_or_default(&form_state, valid_potions.as_slice());
    log_select!(potion_select, Into::<u8>::into(form_state.potion).to_string());
    form.append_child(&potion_select).unwrap();
    debug!("potion select created");
    let potion_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    potion_image.set_id("potion_image");
    potion_image.set_src(form_state.potion.asset().as_str());
    form.append_child(&potion_image).unwrap();
    debug!("potion image created");
    let vessel_label = d.create_element("label").unwrap();
    vessel_label.set_text_content(Some("Vessel "));
    form.append_child(&vessel_label).unwrap();
    debug!("vessel label created");
    let vessel_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    vessel_select.set_id("vessel");
    vessel_select.set_name("vessel");
    let valid_vessels = data::Vessel::valid_for(&form_state);
    valid_vessels.iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_select.append_child(&option).unwrap();});
    form_state.vessel = data::Vessel::resolve_or_default(&form_state, valid_vessels.as_slice());
    log_select!(vessel_select, Into::<u8>::into(form_state.vessel).to_string());
    form.append_child(&vessel_select).unwrap();
    debug!("vessel select created");
    let vessel_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    vessel_image.set_id("vessel_image");
    vessel_image.set_src(form_state.vessel.asset().as_str());
    form.append_child(&vessel_image).unwrap();
    debug!("vessel image created");
    break_line!(d, form);
    debug!("break line created");
    let costs_label = d.create_element("label").unwrap();
    costs_label.set_text_content(Some("Costs "));
    form.append_child(&costs_label).unwrap();
    debug!("costs label created");
    let siege_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    siege_cost_input.set_id("siege_cost");
    siege_cost_input.set_type("number");
    siege_cost_input.set_value(&form_state.siege.cost().to_string());
    siege_cost_input.set_read_only(true);
    form.append_child(&siege_cost_input).unwrap();
    debug!("siege cost input created");
    let heavy_siege_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    heavy_siege_cost_input.set_id("heavy_siege_cost");
    heavy_siege_cost_input.set_type("number");
    heavy_siege_cost_input.set_value(&form_state.heavy_siege.cost().to_string());
    heavy_siege_cost_input.set_read_only(true);
    form.append_child(&heavy_siege_cost_input).unwrap();
    debug!("heavy siege cost input created");
    let pack_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    pack_cost_input.set_id("pack_cost");
    pack_cost_input.set_type("number");
    pack_cost_input.set_value(&form_state.pack.cost().to_string());
    pack_cost_input.set_read_only(true);
    form.append_child(&pack_cost_input).unwrap();
    debug!("pack cost input created");
    let potion_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    potion_cost_input.set_id("potion_cost");
    potion_cost_input.set_type("number");
    potion_cost_input.set_value(&form_state.potion.cost().to_string());
    potion_cost_input.set_read_only(true);
    form.append_child(&potion_cost_input).unwrap();
    debug!("potion cost input created");
    let vessel_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    vessel_cost_input.set_id("vessel_cost");
    vessel_cost_input.set_type("number");
    vessel_cost_input.set_value(&form_state.vessel.cost().to_string());
    vessel_cost_input.set_read_only(true);
    form.append_child(&vessel_cost_input).unwrap();
    debug!("vessel cost input created");
    break_line!(d, form);
    debug!("break line created");
    let weapons_label = d.create_element("label").unwrap();
    weapons_label.set_text_content(Some("Weapons "));
    form.append_child(&weapons_label).unwrap();
    debug!("weapons label created");
    let valid_weapons = data::Weapon::valid_for(&form_state);
    let weapon_1_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_1_select.set_id("weapon_1");
    weapon_1_select.set_name("weapon_1");
    valid_weapons[0].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_1_select.append_child(&option).unwrap();});
    form_state.weapons[0] = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[0].clone(), 0);
    log_select!(weapon_1_select, Into::<u8>::into(form_state.weapons[0]).to_string());
    form.append_child(&weapon_1_select).unwrap();
    debug!("weapon 1 select created");
    let weapon_1_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_1_image.set_id("weapon_1_image");
    weapon_1_image.set_src(form_state.weapons[0].asset().as_str());
    form.append_child(&weapon_1_image).unwrap();
    debug!("weapon 1 image created");
    let weapon_2_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_2_select.set_id("weapon_2");
    weapon_2_select.set_name("weapon_2");
    valid_weapons[1].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_2_select.append_child(&option).unwrap();});
    form_state.weapons[1] = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[1].clone(), 1);
    log_select!(weapon_2_select, Into::<u8>::into(form_state.weapons[1]).to_string());
    form.append_child(&weapon_2_select).unwrap();
    debug!("weapon 2 select created");
    let weapon_2_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_2_image.set_id("weapon_2_image");
    weapon_2_image.set_src(form_state.weapons[1].asset().as_str());
    form.append_child(&weapon_2_image).unwrap();
    debug!("weapon 2 image created");
    let weapon_3_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_3_select.set_id("weapon_3");
    weapon_3_select.set_name("weapon_3");
    valid_weapons[2].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_3_select.append_child(&option).unwrap();});
    form_state.weapons[2] = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[2].clone(), 2);
    log_select!(weapon_3_select, Into::<u8>::into(form_state.weapons[2]).to_string());
    form.append_child(&weapon_3_select).unwrap();
    debug!("weapon 3 select created");
    let weapon_3_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_3_image.set_id("weapon_3_image");
    weapon_3_image.set_src(form_state.weapons[2].asset().as_str());
    form.append_child(&weapon_3_image).unwrap();
    debug!("weapon 3 image created");
    let weapon_4_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_4_select.set_id("weapon_4");
    weapon_4_select.set_name("weapon_4");
    valid_weapons[3].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_4_select.append_child(&option).unwrap();});
    form_state.weapons[3] = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[3].clone(), 3);
    log_select!(weapon_4_select, Into::<u8>::into(form_state.weapons[3]).to_string());
    form.append_child(&weapon_4_select).unwrap();
    debug!("weapon 4 select created");
    let weapon_4_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_4_image.set_id("weapon_4_image");
    weapon_4_image.set_src(form_state.weapons[3].asset().as_str());
    form.append_child(&weapon_4_image).unwrap();
    debug!("weapon 4 image created");
    let weapon_5_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_5_select.set_id("weapon_5");
    weapon_5_select.set_name("weapon_5");
    valid_weapons[4].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_5_select.append_child(&option).unwrap();});
    form_state.weapons[4] = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[4].clone(), 4);
    log_select!(weapon_5_select, Into::<u8>::into(form_state.weapons[4]).to_string());
    form.append_child(&weapon_5_select).unwrap();
    debug!("weapon 5 select created");
    let weapon_5_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_5_image.set_id("weapon_5_image");
    weapon_5_image.set_src(form_state.weapons[4].asset().as_str());
    form.append_child(&weapon_5_image).unwrap();
    debug!("weapon 5 image created");
    break_line!(d, form);
    debug!("break line created");
    let costs_label = d.create_element("label").unwrap();
    costs_label.set_text_content(Some("Costs "));
    form.append_child(&costs_label).unwrap();
    debug!("costs label created");
    let weapon_1_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_1_cost_input.set_id("weapon_1_cost");
    weapon_1_cost_input.set_type("number");
    weapon_1_cost_input.set_value(&form_state.weapons[0].cost().to_string());
    weapon_1_cost_input.set_read_only(true);
    form.append_child(&weapon_1_cost_input).unwrap();
    debug!("weapon 1 cost input created");
    let weapon_2_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_2_cost_input.set_id("weapon_2_cost");
    weapon_2_cost_input.set_type("number");
    weapon_2_cost_input.set_value(&form_state.weapons[1].cost().to_string());
    weapon_2_cost_input.set_read_only(true);
    form.append_child(&weapon_2_cost_input).unwrap();
    debug!("weapon 2 cost input created");
    let weapon_3_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_3_cost_input.set_id("weapon_3_cost");
    weapon_3_cost_input.set_type("number");
    weapon_3_cost_input.set_value(&form_state.weapons[2].cost().to_string());
    weapon_3_cost_input.set_read_only(true);
    form.append_child(&weapon_3_cost_input).unwrap();
    debug!("weapon 3 cost input created");
    let weapon_4_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_4_cost_input.set_id("weapon_4_cost");
    weapon_4_cost_input.set_type("number");
    weapon_4_cost_input.set_value(&form_state.weapons[3].cost().to_string());
    weapon_4_cost_input.set_read_only(true);
    form.append_child(&weapon_4_cost_input).unwrap();
    debug!("weapon 4 cost input created");
    let weapon_5_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_5_cost_input.set_id("weapon_5_cost");
    weapon_5_cost_input.set_type("number");
    weapon_5_cost_input.set_value(&form_state.weapons[4].cost().to_string());
    weapon_5_cost_input.set_read_only(true);
    form.append_child(&weapon_5_cost_input).unwrap();
    debug!("weapon 5 cost input created");
    break_line!(d, form);
    debug!("break line created");
    let weapon_upgrades_label = d.create_element("label").unwrap();
    weapon_upgrades_label.set_text_content(Some("Weapon Upgrades "));
    form.append_child(&weapon_upgrades_label).unwrap();
    debug!("weapon upgrades label created");
    let valid_weapon_upgrades = data::WeaponUpgrade::valid_for(&form_state);
    let weapon_1_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_1_upgrade_select.set_id("weapon_1_upgrade");
    weapon_1_upgrade_select.set_name("weapon_1_upgrade");
    valid_weapon_upgrades[0].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_1_upgrade_select.append_child(&option).unwrap();});
    form_state.weapon_upgrades[0] = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[0].clone(), 0);
    log_select!(weapon_1_upgrade_select, Into::<u8>::into(form_state.weapon_upgrades[0]).to_string());
    form.append_child(&weapon_1_upgrade_select).unwrap();
    debug!("weapon 1 upgrade select created");
    let weapon_1_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_1_upgrade_image.set_id("weapon_1_upgrade_image");
    weapon_1_upgrade_image.set_src(form_state.weapon_upgrades[0].asset().as_str());
    form.append_child(&weapon_1_upgrade_image).unwrap();
    debug!("weapon 1 upgrade image created");
    let weapon_2_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_2_upgrade_select.set_id("weapon_2_upgrade");
    weapon_2_upgrade_select.set_name("weapon_2_upgrade");
    valid_weapon_upgrades[1].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_2_upgrade_select.append_child(&option).unwrap();});
    form_state.weapon_upgrades[1] = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[1].clone(), 1);
    log_select!(weapon_2_upgrade_select, Into::<u8>::into(form_state.weapon_upgrades[1]).to_string());
    form.append_child(&weapon_2_upgrade_select).unwrap();
    debug!("weapon 2 upgrade select created");
    let weapon_2_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_2_upgrade_image.set_id("weapon_2_upgrade_image");
    weapon_2_upgrade_image.set_src(form_state.weapon_upgrades[1].asset().as_str());
    form.append_child(&weapon_2_upgrade_image).unwrap();
    debug!("weapon 2 upgrade image created");
    let weapon_3_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_3_upgrade_select.set_id("weapon_3_upgrade");
    weapon_3_upgrade_select.set_name("weapon_3_upgrade");
    valid_weapon_upgrades[2].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_3_upgrade_select.append_child(&option).unwrap();});
    form_state.weapon_upgrades[2] = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[2].clone(), 2);
    log_select!(weapon_3_upgrade_select, Into::<u8>::into(form_state.weapon_upgrades[2]).to_string());
    form.append_child(&weapon_3_upgrade_select).unwrap();
    debug!("weapon 3 upgrade select created");
    let weapon_3_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_3_upgrade_image.set_id("weapon_3_upgrade_image");
    weapon_3_upgrade_image.set_src(form_state.weapon_upgrades[2].asset().as_str());
    form.append_child(&weapon_3_upgrade_image).unwrap();
    debug!("weapon 3 upgrade image created");
    let weapon_4_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_4_upgrade_select.set_id("weapon_4_upgrade");
    weapon_4_upgrade_select.set_name("weapon_4_upgrade");
    valid_weapon_upgrades[3].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_4_upgrade_select.append_child(&option).unwrap();});
    form_state.weapon_upgrades[3] = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[3].clone(), 3);
    log_select!(weapon_4_upgrade_select, Into::<u8>::into(form_state.weapon_upgrades[3]).to_string());
    form.append_child(&weapon_4_upgrade_select).unwrap();
    debug!("weapon 4 upgrade select created");
    let weapon_4_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_4_upgrade_image.set_id("weapon_4_upgrade_image");
    weapon_4_upgrade_image.set_src(form_state.weapon_upgrades[3].asset().as_str());
    form.append_child(&weapon_4_upgrade_image).unwrap();
    debug!("weapon 4 upgrade image created");
    let weapon_5_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    weapon_5_upgrade_select.set_id("weapon_5_upgrade");
    weapon_5_upgrade_select.set_name("weapon_5_upgrade");
    valid_weapon_upgrades[4].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_5_upgrade_select.append_child(&option).unwrap();});
    form_state.weapon_upgrades[4] = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[4].clone(), 4);
    log_select!(weapon_5_upgrade_select, Into::<u8>::into(form_state.weapon_upgrades[4]).to_string());
    form.append_child(&weapon_5_upgrade_select).unwrap();
    debug!("weapon 5 upgrade select created");
    let weapon_5_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    weapon_5_upgrade_image.set_id("weapon_5_upgrade_image");
    weapon_5_upgrade_image.set_src(form_state.weapon_upgrades[4].asset().as_str());
    form.append_child(&weapon_5_upgrade_image).unwrap();
    debug!("weapon 5 upgrade image created");
    break_line!(d, form);
    debug!("break line created");
    let costs_label = d.create_element("label").unwrap();
    costs_label.set_text_content(Some("Costs "));
    form.append_child(&costs_label).unwrap();
    debug!("costs label created");
    let weapon_1_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_1_upgrade_cost_input.set_id("weapon_1_upgrade_cost");
    weapon_1_upgrade_cost_input.set_type("number");
    weapon_1_upgrade_cost_input.set_value(&form_state.weapon_upgrades[0].cost().to_string());
    weapon_1_upgrade_cost_input.set_read_only(true);
    form.append_child(&weapon_1_upgrade_cost_input).unwrap();
    debug!("weapon 1 upgrade cost input created");
    let weapon_2_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_2_upgrade_cost_input.set_id("weapon_2_upgrade_cost");
    weapon_2_upgrade_cost_input.set_type("number");
    weapon_2_upgrade_cost_input.set_value(&form_state.weapon_upgrades[1].cost().to_string());
    weapon_2_upgrade_cost_input.set_read_only(true);
    form.append_child(&weapon_2_upgrade_cost_input).unwrap();
    debug!("weapon 2 upgrade cost input created");
    let weapon_3_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_3_upgrade_cost_input.set_id("weapon_3_upgrade_cost");
    weapon_3_upgrade_cost_input.set_type("number");
    weapon_3_upgrade_cost_input.set_value(&form_state.weapon_upgrades[2].cost().to_string());
    weapon_3_upgrade_cost_input.set_read_only(true);
    form.append_child(&weapon_3_upgrade_cost_input).unwrap();
    debug!("weapon 3 upgrade cost input created");
    let weapon_4_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_4_upgrade_cost_input.set_id("weapon_4_upgrade_cost");
    weapon_4_upgrade_cost_input.set_type("number");
    weapon_4_upgrade_cost_input.set_value(&form_state.weapon_upgrades[3].cost().to_string());
    weapon_4_upgrade_cost_input.set_read_only(true);
    form.append_child(&weapon_4_upgrade_cost_input).unwrap();
    debug!("weapon 4 upgrade cost input created");
    let weapon_5_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    weapon_5_upgrade_cost_input.set_id("weapon_5_upgrade_cost");
    weapon_5_upgrade_cost_input.set_type("number");
    weapon_5_upgrade_cost_input.set_value(&form_state.weapon_upgrades[4].cost().to_string());
    weapon_5_upgrade_cost_input.set_read_only(true);
    form.append_child(&weapon_5_upgrade_cost_input).unwrap();
    debug!("weapon 5 upgrade cost input created");
    break_line!(d, form);
    debug!("break line created");
    let spells_label = d.create_element("label").unwrap();
    spells_label.set_text_content(Some("Spells "));
    form.append_child(&spells_label).unwrap();
    debug!("spells label created");
    let valid_spells = data::Spell::valid_for(&form_state);
    let spell_1_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    spell_1_select.set_id("spell_1");
    spell_1_select.set_name("spell_1");
    valid_spells[0].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_1_select.append_child(&option).unwrap();});
    form_state.spells[0] = data::Spell::resolve_or_default_slot(&form_state, valid_spells[0].clone(), 0);
    log_select!(spell_1_select, Into::<u8>::into(form_state.spells[0]).to_string());
    form.append_child(&spell_1_select).unwrap();
    debug!("spell 1 select created");
    let spell_1_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    spell_1_image.set_id("spell_1_image");
    spell_1_image.set_src(form_state.spells[0].asset().as_str());
    form.append_child(&spell_1_image).unwrap();
    debug!("spell 1 image created");
    let spell_2_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    spell_2_select.set_id("spell_2");
    spell_2_select.set_name("spell_2");
    valid_spells[1].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_2_select.append_child(&option).unwrap();});
    form_state.spells[1] = data::Spell::resolve_or_default_slot(&form_state, valid_spells[1].clone(), 1);
    log_select!(spell_2_select, Into::<u8>::into(form_state.spells[1]).to_string());
    form.append_child(&spell_2_select).unwrap();
    debug!("spell 2 select created");
    let spell_2_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    spell_2_image.set_id("spell_2_image");
    spell_2_image.set_src(form_state.spells[1].asset().as_str());
    form.append_child(&spell_2_image).unwrap();
    debug!("spell 2 image created");
    let spell_3_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    spell_3_select.set_id("spell_3");
    spell_3_select.set_name("spell_3");
    valid_spells[2].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_3_select.append_child(&option).unwrap();});
    form_state.spells[2] = data::Spell::resolve_or_default_slot(&form_state, valid_spells[2].clone(), 2);
    log_select!(spell_3_select, Into::<u8>::into(form_state.spells[2]).to_string());
    form.append_child(&spell_3_select).unwrap();
    debug!("spell 3 select created");
    let spell_3_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    spell_3_image.set_id("spell_3_image");
    spell_3_image.set_src(form_state.spells[2].asset().as_str());
    form.append_child(&spell_3_image).unwrap();
    debug!("spell 3 image created");
    let spell_4_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    spell_4_select.set_id("spell_4");
    spell_4_select.set_name("spell_4");
    valid_spells[3].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_4_select.append_child(&option).unwrap();});
    form_state.spells[3] = data::Spell::resolve_or_default_slot(&form_state, valid_spells[3].clone(), 3);
    log_select!(spell_4_select, Into::<u8>::into(form_state.spells[3]).to_string());
    form.append_child(&spell_4_select).unwrap();
    debug!("spell 4 select created");
    let spell_4_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    spell_4_image.set_id("spell_4_image");
    spell_4_image.set_src(form_state.spells[3].asset().as_str());
    form.append_child(&spell_4_image).unwrap();
    debug!("spell 4 image created");
    let spell_5_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    spell_5_select.set_id("spell_5");
    spell_5_select.set_name("spell_5");
    valid_spells[4].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_5_select.append_child(&option).unwrap();});
    form_state.spells[4] = data::Spell::resolve_or_default_slot(&form_state, valid_spells[4].clone(), 4);
    log_select!(spell_5_select, Into::<u8>::into(form_state.spells[4]).to_string());
    form.append_child(&spell_5_select).unwrap();
    debug!("spell 5 select created");
    let spell_5_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    spell_5_image.set_id("spell_5_image");
    spell_5_image.set_src(form_state.spells[4].asset().as_str());
    form.append_child(&spell_5_image).unwrap();
    debug!("spell 5 image created");
    break_line!(d, form);
    debug!("break line created");
    let costs_label = d.create_element("label").unwrap();
    costs_label.set_text_content(Some("Costs "));
    form.append_child(&costs_label).unwrap();
    debug!("costs label created");
    let spell_1_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    spell_1_cost_input.set_id("spell_1_cost");
    spell_1_cost_input.set_type("number");
    spell_1_cost_input.set_value(&form_state.spells[0].cost().to_string());
    spell_1_cost_input.set_read_only(true);
    form.append_child(&spell_1_cost_input).unwrap();
    debug!("spell 1 cost input created");
    let spell_2_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    spell_2_cost_input.set_id("spell_2_cost");
    spell_2_cost_input.set_type("number");
    spell_2_cost_input.set_value(&form_state.spells[1].cost().to_string()); 
    spell_2_cost_input.set_read_only(true);
    form.append_child(&spell_2_cost_input).unwrap();
    debug!("spell 2 cost input created");
    let spell_3_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    spell_3_cost_input.set_id("spell_3_cost");
    spell_3_cost_input.set_type("number");
    spell_3_cost_input.set_value(&form_state.spells[2].cost().to_string());
    spell_3_cost_input.set_read_only(true);
    form.append_child(&spell_3_cost_input).unwrap();
    debug!("spell 3 cost input created");
    let spell_4_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    spell_4_cost_input.set_id("spell_4_cost");
    spell_4_cost_input.set_type("number");
    spell_4_cost_input.set_value(&form_state.spells[3].cost().to_string());
    spell_4_cost_input.set_read_only(true);
    form.append_child(&spell_4_cost_input).unwrap();
    debug!("spell 4 cost input created");
    let spell_5_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    spell_5_cost_input.set_id("spell_5_cost");
    spell_5_cost_input.set_type("number");
    spell_5_cost_input.set_value(&form_state.spells[4].cost().to_string());
    spell_5_cost_input.set_read_only(true);
    form.append_child(&spell_5_cost_input).unwrap();
    debug!("spell 5 cost input created");
    break_line!(d, form);
    debug!("break line created");
    let structures_label = d.create_element("label").unwrap();
    structures_label.set_text_content(Some("Structures "));
    form.append_child(&structures_label).unwrap();
    debug!("structures label created");
    let valid_structures = data::Structure::valid_for(&form_state);
    debug!("valid structures: {:?}", valid_structures);
    let structure_1_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    structure_1_select.set_id("structure_1");
    structure_1_select.set_name("structure_1");
    valid_structures[0].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_1_select.append_child(&option).unwrap();});
    form_state.structures[0] = data::Structure::resolve_or_default_slot(&form_state, valid_structures[0].clone(), 0);
    log_select!(structure_1_select, Into::<u8>::into(form_state.structures[0]).to_string());
    form.append_child(&structure_1_select).unwrap();
    debug!("structure 1 select created");
    let structure_1_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    structure_1_image.set_id("structure_1_image");
    structure_1_image.set_src(form_state.structures[0].asset().as_str());
    form.append_child(&structure_1_image).unwrap();
    debug!("structure 1 image created");
    let structure_2_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    structure_2_select.set_id("structure_2");
    structure_2_select.set_name("structure_2");
    valid_structures[1].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_2_select.append_child(&option).unwrap();});
    form_state.structures[1] = data::Structure::resolve_or_default_slot(&form_state, valid_structures[1].clone(), 1);
    log_select!(structure_2_select, Into::<u8>::into(form_state.structures[1]).to_string());
    form.append_child(&structure_2_select).unwrap();
    debug!("structure 2 select created");
    let structure_2_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    structure_2_image.set_id("structure_2_image");
    structure_2_image.set_src(form_state.structures[1].asset().as_str());
    form.append_child(&structure_2_image).unwrap();
    debug!("structure 2 image created");
    let structure_3_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    structure_3_select.set_id("structure_3");
    structure_3_select.set_name("structure_3");
    valid_structures[2].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_3_select.append_child(&option).unwrap();});
    form_state.structures[2] = data::Structure::resolve_or_default_slot(&form_state, valid_structures[2].clone(), 2);
    log_select!(structure_3_select, Into::<u8>::into(form_state.structures[2]).to_string());
    form.append_child(&structure_3_select).unwrap();
    debug!("structure 3 select created");
    let structure_3_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    structure_3_image.set_id("structure_3_image");
    structure_3_image.set_src(form_state.structures[2].asset().as_str());
    form.append_child(&structure_3_image).unwrap();
    debug!("structure 3 image created");
    let structure_4_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    structure_4_select.set_id("structure_4");
    structure_4_select.set_name("structure_4");
    valid_structures[3].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_4_select.append_child(&option).unwrap();});
    form_state.structures[3] = data::Structure::resolve_or_default_slot(&form_state, valid_structures[3].clone(), 3);
    log_select!(structure_4_select, Into::<u8>::into(form_state.structures[3]).to_string());
    form.append_child(&structure_4_select).unwrap();
    debug!("structure 4 select created");
    let structure_4_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    structure_4_image.set_id("structure_4_image");
    structure_4_image.set_src(form_state.structures[3].asset().as_str());
    form.append_child(&structure_4_image).unwrap();
    debug!("structure 4 image created");
    let structure_5_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    structure_5_select.set_id("structure_5");
    structure_5_select.set_name("structure_5");
    valid_structures[4].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_5_select.append_child(&option).unwrap();});
    form_state.structures[4] = data::Structure::resolve_or_default_slot(&form_state, valid_structures[4].clone(), 4);
    log_select!(structure_5_select, Into::<u8>::into(form_state.structures[4]).to_string());
    form.append_child(&structure_5_select).unwrap();
    debug!("structure 5 select created");
    let structure_5_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    structure_5_image.set_id("structure_5_image");
    structure_5_image.set_src(form_state.structures[4].asset().as_str());
    form.append_child(&structure_5_image).unwrap();
    break_line!(d, form);
    debug!("break line created");
    let costs_label = d.create_element("label").unwrap();
    costs_label.set_text_content(Some("Costs "));
    form.append_child(&costs_label).unwrap();
    debug!("costs label created");
    let structure_1_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    structure_1_cost_input.set_id("structure_1_cost");
    structure_1_cost_input.set_type("number");
    structure_1_cost_input.set_value(&form_state.structures[0].cost().to_string());
    structure_1_cost_input.set_read_only(true);
    form.append_child(&structure_1_cost_input).unwrap();
    debug!("structure 1 cost input created");
    let structure_2_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    structure_2_cost_input.set_id("structure_2_cost");
    structure_2_cost_input.set_type("number");
    structure_2_cost_input.set_value(&form_state.structures[1].cost().to_string());
    structure_2_cost_input.set_read_only(true);
    form.append_child(&structure_2_cost_input).unwrap();
    debug!("structure 2 cost input created");
    let structure_3_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    structure_3_cost_input.set_id("structure_3_cost");
    structure_3_cost_input.set_type("number");
    structure_3_cost_input.set_value(&form_state.structures[2].cost().to_string());
    structure_3_cost_input.set_read_only(true);
    form.append_child(&structure_3_cost_input).unwrap();
    debug!("structure 3 cost input created");
    let structure_4_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    structure_4_cost_input.set_id("structure_4_cost");
    structure_4_cost_input.set_type("number");
    structure_4_cost_input.set_value(&form_state.structures[3].cost().to_string());
    structure_4_cost_input.set_read_only(true);
    form.append_child(&structure_4_cost_input).unwrap();
    debug!("structure 4 cost input created");
    let structure_5_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    structure_5_cost_input.set_id("structure_5_cost");
    structure_5_cost_input.set_type("number");
    structure_5_cost_input.set_value(&form_state.structures[4].cost().to_string());
    structure_5_cost_input.set_read_only(true);
    form.append_child(&structure_5_cost_input).unwrap();
    debug!("structure 5 cost input created");
    break_line!(d, form);
    debug!("break line created");
    let vessel_upgrades_label = d.create_element("label").unwrap();
    vessel_upgrades_label.set_text_content(Some("Vessel Upgrades "));
    form.append_child(&vessel_upgrades_label).unwrap();
    debug!("vessel upgrades label created");
    let valid_vessel_upgrades = data::VesselUpgrade::valid_for(&form_state);
    let vessel_1_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    vessel_1_upgrade_select.set_id("vessel_1_upgrade");
    vessel_1_upgrade_select.set_name("vessel_1_upgrade");
    valid_vessel_upgrades[0].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_1_upgrade_select.append_child(&option).unwrap();});
    form_state.vessel_upgrades[0] = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[0].clone(), 0);
    log_select!(vessel_1_upgrade_select, Into::<u8>::into(form_state.vessel_upgrades[0]).to_string());
    form.append_child(&vessel_1_upgrade_select).unwrap();
    debug!("vessel 1 upgrade select created");
    let vessel_1_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    vessel_1_upgrade_image.set_id("vessel_1_upgrade_image");
    vessel_1_upgrade_image.set_src(form_state.vessel_upgrades[0].asset().as_str());
    form.append_child(&vessel_1_upgrade_image).unwrap();
    debug!("vessel 1 upgrade image created");
    let vessel_2_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    vessel_2_upgrade_select.set_id("vessel_2_upgrade");
    vessel_2_upgrade_select.set_name("vessel_2_upgrade");
    valid_vessel_upgrades[1].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_2_upgrade_select.append_child(&option).unwrap();});
    form_state.vessel_upgrades[1] = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[1].clone(), 1);
    log_select!(vessel_2_upgrade_select, Into::<u8>::into(form_state.vessel_upgrades[1]).to_string());
    form.append_child(&vessel_2_upgrade_select).unwrap();
    debug!("vessel 2 upgrade select created");
    let vessel_2_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    vessel_2_upgrade_image.set_id("vessel_2_upgrade_image");
    vessel_2_upgrade_image.set_src(form_state.vessel_upgrades[1].asset().as_str());
    form.append_child(&vessel_2_upgrade_image).unwrap();
    debug!("vessel 2 upgrade image created");
    let vessel_3_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    vessel_3_upgrade_select.set_id("vessel_3_upgrade");
    vessel_3_upgrade_select.set_name("vessel_3_upgrade");
    valid_vessel_upgrades[2].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_3_upgrade_select.append_child(&option).unwrap();});
    form_state.vessel_upgrades[2] = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[2].clone(), 2);
    log_select!(vessel_3_upgrade_select, Into::<u8>::into(form_state.vessel_upgrades[2]).to_string());
    form.append_child(&vessel_3_upgrade_select).unwrap();
    debug!("vessel 3 upgrade select created");
    let vessel_3_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    vessel_3_upgrade_image.set_id("vessel_3_upgrade_image");
    vessel_3_upgrade_image.set_src(form_state.vessel_upgrades[2].asset().as_str());
    form.append_child(&vessel_3_upgrade_image).unwrap();
    debug!("vessel 3 upgrade image created");
    let vessel_4_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    vessel_4_upgrade_select.set_id("vessel_4_upgrade");
    vessel_4_upgrade_select.set_name("vessel_4_upgrade");
    valid_vessel_upgrades[3].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_4_upgrade_select.append_child(&option).unwrap();});
    form_state.vessel_upgrades[3] = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[3].clone(), 3);
    log_select!(vessel_4_upgrade_select, Into::<u8>::into(form_state.vessel_upgrades[3]).to_string());
    form.append_child(&vessel_4_upgrade_select).unwrap();
    debug!("vessel 4 upgrade select created");
    let vessel_4_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    vessel_4_upgrade_image.set_id("vessel_4_upgrade_image");
    vessel_4_upgrade_image.set_src(form_state.vessel_upgrades[3].asset().as_str());
    form.append_child(&vessel_4_upgrade_image).unwrap();
    debug!("vessel 4 upgrade image created");
    let vessel_5_upgrade_select = d.create_element("select").unwrap().unchecked_into::<HtmlSelectElement>();
    vessel_5_upgrade_select.set_id("vessel_5_upgrade");
    vessel_5_upgrade_select.set_name("vessel_5_upgrade");
    valid_vessel_upgrades[4].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_5_upgrade_select.append_child(&option).unwrap();});
    form_state.vessel_upgrades[4] = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[4].clone(), 4);
    log_select!(vessel_5_upgrade_select, Into::<u8>::into(form_state.vessel_upgrades[4]).to_string());
    form.append_child(&vessel_5_upgrade_select).unwrap();
    debug!("vessel 5 upgrade select created");
    let vessel_5_upgrade_image = d.create_element("img").unwrap().unchecked_into::<HtmlImageElement>();
    vessel_5_upgrade_image.set_id("vessel_5_upgrade_image");
    vessel_5_upgrade_image.set_src(form_state.vessel_upgrades[4].asset().as_str());
    form.append_child(&vessel_5_upgrade_image).unwrap();
    debug!("vessel 5 upgrade image created");
    break_line!(d, form);
    debug!("break line created");
    let costs_label = d.create_element("label").unwrap();
    costs_label.set_text_content(Some("Costs "));
    form.append_child(&costs_label).unwrap();
    debug!("costs label created");
    let vessel_1_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    vessel_1_upgrade_cost_input.set_id("vessel_1_upgrade_cost");
    vessel_1_upgrade_cost_input.set_type("number");
    vessel_1_upgrade_cost_input.set_value(&form_state.vessel_upgrades[0].cost().to_string());
    vessel_1_upgrade_cost_input.set_read_only(true);
    form.append_child(&vessel_1_upgrade_cost_input).unwrap();
    debug!("vessel 1 upgrade cost input created");
    let vessel_2_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    vessel_2_upgrade_cost_input.set_id("vessel_2_upgrade_cost");
    vessel_2_upgrade_cost_input.set_type("number");
    vessel_2_upgrade_cost_input.set_value(&form_state.vessel_upgrades[1].cost().to_string());
    vessel_2_upgrade_cost_input.set_read_only(true);
    form.append_child(&vessel_2_upgrade_cost_input).unwrap();
    debug!("vessel 2 upgrade cost input created");
    let vessel_3_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    vessel_3_upgrade_cost_input.set_id("vessel_3_upgrade_cost");
    vessel_3_upgrade_cost_input.set_type("number");
    vessel_3_upgrade_cost_input.set_value(&form_state.vessel_upgrades[2].cost().to_string());
    vessel_3_upgrade_cost_input.set_read_only(true);
    form.append_child(&vessel_3_upgrade_cost_input).unwrap();
    debug!("vessel 3 upgrade cost input created");
    let vessel_4_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    vessel_4_upgrade_cost_input.set_id("vessel_4_upgrade_cost");
    vessel_4_upgrade_cost_input.set_type("number");
    vessel_4_upgrade_cost_input.set_value(&form_state.vessel_upgrades[3].cost().to_string());
    vessel_4_upgrade_cost_input.set_read_only(true);
    form.append_child(&vessel_4_upgrade_cost_input).unwrap();
    debug!("vessel 4 upgrade cost input created");
    let vessel_5_upgrade_cost_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
    vessel_5_upgrade_cost_input.set_id("vessel_5_upgrade_cost");
    vessel_5_upgrade_cost_input.set_type("number");
    vessel_5_upgrade_cost_input.set_value(&form_state.vessel_upgrades[4].cost().to_string());
    vessel_5_upgrade_cost_input.set_read_only(true);
    form.append_child(&vessel_5_upgrade_cost_input).unwrap();
    debug!("vessel 5 upgrade cost input created");

    let costs = form_state.costs();
    let base_cost = i32::from(form_state.unit_type.cost());
    let base_gold = i32::from(form_state.unit_type.unit_gold());
    let refit_cost = i32::from(form_state.refit_type.cost());
    let refit_gold = i32::from(form_state.refit_type.unit_gold());

    let a = allocate(&costs, base_gold, refit_gold, form_state.refit_type != data::RefitType::None);

    let item_total = costs.iter().sum::<i32>();
    let company_used = base_cost + refit_cost + (item_total - a.base_used - a.refit_used);
    let remaining_budget = i32::from(form_state.budget) - company_used;
    let unit_gold_remaining = (base_gold - a.base_used) + (refit_gold - a.refit_used);

    remaining_budget_input.set_value(remaining_budget.to_string().as_str());
    unit_cost_input.set_value((base_cost + refit_cost + item_total).to_string().as_str());
    unit_gold_remaining_input.set_value(unit_gold_remaining.to_string().as_str());
    info!("budget {} company_used {} remaining {} unit_gold_remaining {}", form_state.budget, company_used, remaining_budget, unit_gold_remaining);
    debug!("allocation base_used {} refit_used {} refit_ok {}", a.base_used, a.refit_used, a.refit_ok);
    if remaining_budget < 0 {
        warn!("company cost {} exceeds budget {}", company_used, form_state.budget);
        remaining_budget_input.set_class_name("bad");
    } else {
        remaining_budget_input.set_class_name("");
    }
    if a.refit_ok {
        refit_type_select.set_class_name("");
    } else {
        warn!("refit gold cannot be spent exactly");
        refit_type_select.set_class_name("bad");
    }

    let oninput_closure = Closure::<dyn FnMut(web_sys::Event)>::new(oninput);
    form.set_oninput(Some(oninput_closure.as_ref().unchecked_ref()));
    oninput_closure.forget();
    debug!("oninput closure created");

    horizontal_rule!(d, body);
    if let Some(local_storage) = local_storage
        && local_storage.set_item("_test", "test").is_ok()
        && local_storage.get_item("_test").is_ok()
    {
        info!("local storage is usable, creating save and load controls");

        let save_closure = Closure::<dyn FnMut(web_sys::Event)>::new(save);
        
        debug!("break line created");
        let save_1 = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        save_1.set_id("save_1");
        save_1.set_text_content(Some("Save 1"));
        save_1.set_oninput(Some(save_closure.as_ref().unchecked_ref()));
        body.append_child(&save_1).unwrap();
        debug!("save 1 button created");
        let save_2 = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        save_2.set_id("save_2");
        save_2.set_text_content(Some("Save 2"));
        save_2.set_oninput(Some(save_closure.as_ref().unchecked_ref()));
        body.append_child(&save_2).unwrap();
        debug!("save 2 button created");
        let save_3 = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        save_3.set_id("save_3");
        save_3.set_text_content(Some("Save 3"));
        save_3.set_oninput(Some(save_closure.as_ref().unchecked_ref()));
        body.append_child(&save_3).unwrap();
        debug!("save 3 button created");
        let save_4 = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        save_4.set_id("save_4");
        save_4.set_text_content(Some("Save 4"));
        save_4.set_oninput(Some(save_closure.as_ref().unchecked_ref()));
        body.append_child(&save_4).unwrap();
        debug!("save 4 button created");
        let save_5 = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        save_5.set_id("save_5");
        save_5.set_text_content(Some("Save 5"));
        save_5.set_oninput(Some(save_closure.as_ref().unchecked_ref()));
        body.append_child(&save_5).unwrap();
        debug!("save 5 button created");
        save_closure.forget();
        debug!("save closure forgotten");
        break_line!(d, body);
        let load_1_form = d.create_element("form").unwrap().unchecked_into::<HtmlFormElement>();
        let load_1_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
        let load_1_button = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        load_1_form.append_child(&load_1_input).unwrap();
        load_1_input.set_id("load_1_input");
        load_1_input.set_name("L");
        load_1_input.set_value(stored_save(&local_storage, "crusadercalc:save_1").as_str());
        load_1_input.set_hidden(true);
        load_1_form.append_child(&load_1_button).unwrap();
        load_1_button.set_text_content(Some("Load 1"));
        load_1_button.set_type("submit");
        body.append_child(&load_1_form).unwrap();
        load_1_form.style().set_property("display", "inline").unwrap();
        debug!("load 1 form created");
        let load_2_form = d.create_element("form").unwrap().unchecked_into::<HtmlFormElement>();
        let load_2_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
        let load_2_button = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        load_2_form.append_child(&load_2_input).unwrap();
        load_2_input.set_id("load_2_input");
        load_2_input.set_name("L");
        load_2_input.set_value(stored_save(&local_storage, "crusadercalc:save_2").as_str());
        load_2_input.set_hidden(true);
        load_2_form.append_child(&load_2_button).unwrap();
        load_2_button.set_text_content(Some("Load 2"));
        load_2_button.set_type("submit");
        body.append_child(&load_2_form).unwrap();
        load_2_form.style().set_property("display", "inline").unwrap();
        debug!("load 2 form created");
        let load_3_form = d.create_element("form").unwrap().unchecked_into::<HtmlFormElement>();
        let load_3_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
        let load_3_button = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        load_3_form.append_child(&load_3_input).unwrap();
        load_3_input.set_id("load_3_input");
        load_3_input.set_name("L");
        load_3_input.set_value(stored_save(&local_storage, "crusadercalc:save_3").as_str());
        load_3_input.set_hidden(true);
        load_3_form.append_child(&load_3_button).unwrap();
        load_3_button.set_text_content(Some("Load 3"));
        load_3_button.set_type("submit");
        body.append_child(&load_3_form).unwrap();
        load_3_form.style().set_property("display", "inline").unwrap();
        debug!("load 3 form created");
        let load_4_form = d.create_element("form").unwrap().unchecked_into::<HtmlFormElement>();
        let load_4_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
        let load_4_button = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        load_4_form.append_child(&load_4_input).unwrap();
        load_4_input.set_id("load_4_input");
        load_4_input.set_name("L");
        load_4_input.set_value(stored_save(&local_storage, "crusadercalc:save_4").as_str());
        load_4_input.set_hidden(true);
        load_4_form.append_child(&load_4_button).unwrap();
        load_4_button.set_text_content(Some("Load 4"));
        load_4_button.set_type("submit");
        body.append_child(&load_4_form).unwrap();
        load_4_form.style().set_property("display", "inline").unwrap();
        debug!("load 4 form created");
        let load_5_form = d.create_element("form").unwrap().unchecked_into::<HtmlFormElement>();
        let load_5_input = d.create_element("input").unwrap().unchecked_into::<HtmlInputElement>();
        let load_5_button = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        load_5_form.append_child(&load_5_input).unwrap();
        load_5_input.set_id("load_5_input");
        load_5_input.set_name("L");
        load_5_input.set_value(stored_save(&local_storage, "crusadercalc:save_5").as_str());
        load_5_input.set_hidden(true);
        load_5_form.append_child(&load_5_button).unwrap();
        load_5_button.set_text_content(Some("Load 5"));
        load_5_button.set_type("submit");
        body.append_child(&load_5_form).unwrap();
        load_5_form.style().set_property("display", "inline").unwrap();
        debug!("load 5 form created");
        let copy_button = d.create_element("button").unwrap().unchecked_into::<HtmlButtonElement>();
        copy_button.set_id("copy_button");
        copy_button.set_text_content(Some("Copy"));
        let code: [u8; 35] = form_state.into();
        let code_string = URL_SAFE_NO_PAD.encode(code);
        copy_button.set_attribute("onclick",   format!(
            "navigator.clipboard.writeText(location.origin+location.pathname+'?L={}')",
            code_string
        ).as_str()).unwrap();
        body.append_child(&copy_button).unwrap();
        debug!("copy button created");
        if cfg!(debug_assertions) {
            let debug_text = d.create_element("p").unwrap().unchecked_into::<HtmlParagraphElement>();
            debug_text.set_id("debug_text");
            debug_text.set_text_content(Some(format!("Code: {}", code_string).as_str()));
            body.append_child(&debug_text).unwrap();
            debug!("debug text created");
        }
    } else {
        warn!("local storage is not usable, save and load controls were not created");
    }

}

fn oninput(_: web_sys::Event) {
    let form = web_sys::window().unwrap().document().unwrap().get_element_by_id("form").unwrap().unchecked_into::<HtmlFormElement>();
    let form_data = match web_sys::FormData::new_with_form(&form) {
        Ok(data) => data,
        Err(err) => {
            error!("failed to read the form: {:?}", err);
            return;
        }
    };
    let form_state: Result<FormState, &'static str> = form_data.try_into();
    if let Err(e) = form_state {
        let bad_element = get_element_by_id!(e);
        bad_element.set_class_name("error");
        error!("parsing form state error: {}", e);
        return;
    }
    get_element_by_id!("gold_budget").set_class_name("");
    debug!("form state: {:?}", form_state);
    let mut form_state = form_state.unwrap();
    info!("form state: {:?}", form_state);
    let valid_unit_types = data::UnitType::valid_for(&form_state);
    let unit_type_select = get_element_by_id!("unit_type").unchecked_into::<HtmlSelectElement>();
    let new_unit_type = data::UnitType::resolve_or_default(&form_state, valid_unit_types.as_slice());
    unit_type_select.set_inner_html("");
    valid_unit_types.iter().map(|&ut| Into::<HtmlOptionElement>::into(ut)).for_each(|option| {unit_type_select.append_child(&option).unwrap();});
    log_select!(unit_type_select, Into::<u8>::into(new_unit_type).to_string());
    form_state.unit_type = new_unit_type;

    let valid_refit_types = data::RefitType::valid_for(&form_state);
    let refit_type_select = get_element_by_id!("refit_type").unchecked_into::<HtmlSelectElement>();
    let new_refit_type = data::RefitType::resolve_or_default(&form_state, valid_refit_types.as_slice());
    refit_type_select.set_inner_html("");
    valid_refit_types.iter().map(|&rt| Into::<HtmlOptionElement>::into(rt)).for_each(|option| {refit_type_select.append_child(&option).unwrap();});
    log_select!(refit_type_select, Into::<u8>::into(new_refit_type).to_string());
    form_state.refit_type = new_refit_type;

    let valid_sieges = data::Siege::valid_for(&form_state);
    let siege_select = get_element_by_id!("siege").unchecked_into::<HtmlSelectElement>();
    let new_siege = data::Siege::resolve_or_default(&form_state, valid_sieges.as_slice());
    siege_select.set_inner_html("");
    valid_sieges.iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {siege_select.append_child(&option).unwrap();});
    log_select!(siege_select, Into::<u8>::into(new_siege).to_string());
    form_state.siege = new_siege;

    let valid_heavy_sieges = data::HeavySiege::valid_for(&form_state);
    let heavy_siege_select = get_element_by_id!("heavy_siege").unchecked_into::<HtmlSelectElement>();
    let new_heavy_siege = data::HeavySiege::resolve_or_default(&form_state, valid_heavy_sieges.as_slice());
    heavy_siege_select.set_inner_html("");
    valid_heavy_sieges.iter().map(|&hs| Into::<HtmlOptionElement>::into(hs)).for_each(|option| {heavy_siege_select.append_child(&option).unwrap();});
    log_select!(heavy_siege_select, Into::<u8>::into(new_heavy_siege).to_string());
    form_state.heavy_siege = new_heavy_siege;

    let valid_packs = data::Pack::valid_for(&form_state);
    let pack_select = get_element_by_id!("pack").unchecked_into::<HtmlSelectElement>();
    let new_pack = data::Pack::resolve_or_default(&form_state, valid_packs.as_slice());
    pack_select.set_inner_html("");
    valid_packs.iter().map(|&p| Into::<HtmlOptionElement>::into(p)).for_each(|option| {pack_select.append_child(&option).unwrap();});
    log_select!(pack_select, Into::<u8>::into(new_pack).to_string());
    form_state.pack = new_pack;
    
    let valid_potions = data::Potion::valid_for(&form_state);
    let potion_select = get_element_by_id!("potion").unchecked_into::<HtmlSelectElement>();
    let new_potion = data::Potion::resolve_or_default(&form_state, valid_potions.as_slice());
    potion_select.set_inner_html("");
    valid_potions.iter().map(|&p| Into::<HtmlOptionElement>::into(p)).for_each(|option| {potion_select.append_child(&option).unwrap();});
    log_select!(potion_select, Into::<u8>::into(new_potion).to_string());
    form_state.potion = new_potion;
    
    let valid_vessels = data::Vessel::valid_for(&form_state);
    let vessel_select = get_element_by_id!("vessel").unchecked_into::<HtmlSelectElement>();
    let new_vessel = data::Vessel::resolve_or_default(&form_state, valid_vessels.as_slice());
    vessel_select.set_inner_html("");
    valid_vessels.iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_select.append_child(&option).unwrap();});
    log_select!(vessel_select, Into::<u8>::into(new_vessel).to_string());
    form_state.vessel = new_vessel;
    
    let valid_weapons = data::Weapon::valid_for(&form_state);
    let weapon_1_select = get_element_by_id!("weapon_1").unchecked_into::<HtmlSelectElement>();
    let new_weapon_1 = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[0].clone(), 0);
    weapon_1_select.set_inner_html("");
    valid_weapons[0].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_1_select.append_child(&option).unwrap();});
    log_select!(weapon_1_select, Into::<u8>::into(new_weapon_1).to_string());
    form_state.weapons[0] = new_weapon_1;

    let weapon_2_select = get_element_by_id!("weapon_2").unchecked_into::<HtmlSelectElement>();
    let new_weapon_2 = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[1].clone(), 1);
    weapon_2_select.set_inner_html("");
    valid_weapons[1].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_2_select.append_child(&option).unwrap();});
    log_select!(weapon_2_select, Into::<u8>::into(new_weapon_2).to_string());
    form_state.weapons[1] = new_weapon_2;

    let weapon_3_select = get_element_by_id!("weapon_3").unchecked_into::<HtmlSelectElement>();
    let new_weapon_3 = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[2].clone(), 2);
    weapon_3_select.set_inner_html("");
    valid_weapons[2].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_3_select.append_child(&option).unwrap();});
    log_select!(weapon_3_select, Into::<u8>::into(new_weapon_3).to_string());
    form_state.weapons[2] = new_weapon_3;
    
    let weapon_4_select = get_element_by_id!("weapon_4").unchecked_into::<HtmlSelectElement>();
    let new_weapon_4 = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[3].clone(), 3);
    weapon_4_select.set_inner_html("");
    valid_weapons[3].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_4_select.append_child(&option).unwrap();});
    log_select!(weapon_4_select, Into::<u8>::into(new_weapon_4).to_string());
    form_state.weapons[3] = new_weapon_4;
    
    let weapon_5_select = get_element_by_id!("weapon_5").unchecked_into::<HtmlSelectElement>();
    let new_weapon_5 = data::Weapon::resolve_or_default_slot(&form_state, valid_weapons[4].clone(), 4);
    weapon_5_select.set_inner_html("");
    valid_weapons[4].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_5_select.append_child(&option).unwrap();});
    log_select!(weapon_5_select, Into::<u8>::into(new_weapon_5).to_string());
    form_state.weapons[4] = new_weapon_5;
    
    let valid_weapon_upgrades = data::WeaponUpgrade::valid_for(&form_state);
    let weapon_1_upgrade_select = get_element_by_id!("weapon_1_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_weapon_1_upgrade = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[0].clone(), 0);
    weapon_1_upgrade_select.set_inner_html("");
    valid_weapon_upgrades[0].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_1_upgrade_select.append_child(&option).unwrap();});
    log_select!(weapon_1_upgrade_select, Into::<u8>::into(new_weapon_1_upgrade).to_string());
    form_state.weapon_upgrades[0] = new_weapon_1_upgrade;
    
    let weapon_2_upgrade_select = get_element_by_id!("weapon_2_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_weapon_2_upgrade = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[1].clone(), 1);
    weapon_2_upgrade_select.set_inner_html("");
    valid_weapon_upgrades[1].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_2_upgrade_select.append_child(&option).unwrap();});
    log_select!(weapon_2_upgrade_select, Into::<u8>::into(new_weapon_2_upgrade).to_string());
    form_state.weapon_upgrades[1] = new_weapon_2_upgrade;
    
    let weapon_3_upgrade_select = get_element_by_id!("weapon_3_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_weapon_3_upgrade = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[2].clone(), 2);
    weapon_3_upgrade_select.set_inner_html("");
    valid_weapon_upgrades[2].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_3_upgrade_select.append_child(&option).unwrap();});
    log_select!(weapon_3_upgrade_select, Into::<u8>::into(new_weapon_3_upgrade).to_string());
    form_state.weapon_upgrades[2] = new_weapon_3_upgrade;
    
    let weapon_4_upgrade_select = get_element_by_id!("weapon_4_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_weapon_4_upgrade = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[3].clone(), 3);
    weapon_4_upgrade_select.set_inner_html("");
    valid_weapon_upgrades[3].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_4_upgrade_select.append_child(&option).unwrap();});
    log_select!(weapon_4_upgrade_select, Into::<u8>::into(new_weapon_4_upgrade).to_string());
    form_state.weapon_upgrades[3] = new_weapon_4_upgrade;
    
    let weapon_5_upgrade_select = get_element_by_id!("weapon_5_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_weapon_5_upgrade = data::WeaponUpgrade::resolve_or_default_slot(&form_state, valid_weapon_upgrades[4].clone(), 4);
    weapon_5_upgrade_select.set_inner_html("");
    valid_weapon_upgrades[4].iter().map(|&w| Into::<HtmlOptionElement>::into(w)).for_each(|option| {weapon_5_upgrade_select.append_child(&option).unwrap();});
    log_select!(weapon_5_upgrade_select, Into::<u8>::into(new_weapon_5_upgrade).to_string());
    form_state.weapon_upgrades[4] = new_weapon_5_upgrade;

    let valid_spells = data::Spell::valid_for(&form_state);
    let spell_1_select = get_element_by_id!("spell_1").unchecked_into::<HtmlSelectElement>();
    let new_spell_1 = data::Spell::resolve_or_default_slot(&form_state, valid_spells[0].clone(), 0);
    spell_1_select.set_inner_html("");
    valid_spells[0].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_1_select.append_child(&option).unwrap();});
    log_select!(spell_1_select, Into::<u8>::into(new_spell_1).to_string());
    form_state.spells[0] = new_spell_1;
    
    let spell_2_select = get_element_by_id!("spell_2").unchecked_into::<HtmlSelectElement>();
    let new_spell_2 = data::Spell::resolve_or_default_slot(&form_state, valid_spells[1].clone(), 1);
    spell_2_select.set_inner_html("");
    valid_spells[1].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_2_select.append_child(&option).unwrap();});
    log_select!(spell_2_select, Into::<u8>::into(new_spell_2).to_string());
    form_state.spells[1] = new_spell_2;
    
    let spell_3_select = get_element_by_id!("spell_3").unchecked_into::<HtmlSelectElement>();
    let new_spell_3 = data::Spell::resolve_or_default_slot(&form_state, valid_spells[2].clone(), 2);
    spell_3_select.set_inner_html("");
    valid_spells[2].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_3_select.append_child(&option).unwrap();});
    log_select!(spell_3_select, Into::<u8>::into(new_spell_3).to_string());
    form_state.spells[2] = new_spell_3;
    
    let spell_4_select = get_element_by_id!("spell_4").unchecked_into::<HtmlSelectElement>();
    let new_spell_4 = data::Spell::resolve_or_default_slot(&form_state, valid_spells[3].clone(), 3);
    spell_4_select.set_inner_html("");
    valid_spells[3].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_4_select.append_child(&option).unwrap();});
    log_select!(spell_4_select, Into::<u8>::into(new_spell_4).to_string());
    form_state.spells[3] = new_spell_4;
    
    let spell_5_select = get_element_by_id!("spell_5").unchecked_into::<HtmlSelectElement>();
    let new_spell_5 = data::Spell::resolve_or_default_slot(&form_state, valid_spells[4].clone(), 4);
    spell_5_select.set_inner_html("");
    valid_spells[4].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {spell_5_select.append_child(&option).unwrap();});
    log_select!(spell_5_select, Into::<u8>::into(new_spell_5).to_string());
    form_state.spells[4] = new_spell_5;
    
    let valid_structures = data::Structure::valid_for(&form_state);
    let structure_1_select = get_element_by_id!("structure_1").unchecked_into::<HtmlSelectElement>();
    let new_structure_1 = data::Structure::resolve_or_default_slot(&form_state, valid_structures[0].clone(), 0);
    structure_1_select.set_inner_html("");
    valid_structures[0].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_1_select.append_child(&option).unwrap();});
    log_select!(structure_1_select, Into::<u8>::into(new_structure_1).to_string());
    form_state.structures[0] = new_structure_1;
    
    let structure_2_select = get_element_by_id!("structure_2").unchecked_into::<HtmlSelectElement>();
    let new_structure_2 = data::Structure::resolve_or_default_slot(&form_state, valid_structures[1].clone(), 1);
    structure_2_select.set_inner_html("");
    valid_structures[1].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_2_select.append_child(&option).unwrap();});
    log_select!(structure_2_select, Into::<u8>::into(new_structure_2).to_string());
    form_state.structures[1] = new_structure_2;
    
    let structure_3_select = get_element_by_id!("structure_3").unchecked_into::<HtmlSelectElement>();
    let new_structure_3 = data::Structure::resolve_or_default_slot(&form_state, valid_structures[2].clone(), 2);
    structure_3_select.set_inner_html("");
    valid_structures[2].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_3_select.append_child(&option).unwrap();});
    log_select!(structure_3_select, Into::<u8>::into(new_structure_3).to_string());
    form_state.structures[2] = new_structure_3;
    
    let structure_4_select = get_element_by_id!("structure_4").unchecked_into::<HtmlSelectElement>();
    let new_structure_4 = data::Structure::resolve_or_default_slot(&form_state, valid_structures[3].clone(), 3);
    structure_4_select.set_inner_html("");
    valid_structures[3].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_4_select.append_child(&option).unwrap();});
    log_select!(structure_4_select, Into::<u8>::into(new_structure_4).to_string());
    form_state.structures[3] = new_structure_4;
    
    let structure_5_select = get_element_by_id!("structure_5").unchecked_into::<HtmlSelectElement>();
    let new_structure_5 = data::Structure::resolve_or_default_slot(&form_state, valid_structures[4].clone(), 4);
    structure_5_select.set_inner_html("");
    valid_structures[4].iter().map(|&s| Into::<HtmlOptionElement>::into(s)).for_each(|option| {structure_5_select.append_child(&option).unwrap();});
    log_select!(structure_5_select, Into::<u8>::into(new_structure_5).to_string());
    form_state.structures[4] = new_structure_5;
    
    let valid_vessel_upgrades = data::VesselUpgrade::valid_for(&form_state);
    let vessel_1_upgrade_select = get_element_by_id!("vessel_1_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_vessel_1_upgrade = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[0].clone(), 0);
    vessel_1_upgrade_select.set_inner_html("");
    valid_vessel_upgrades[0].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_1_upgrade_select.append_child(&option).unwrap();});
    log_select!(vessel_1_upgrade_select, Into::<u8>::into(new_vessel_1_upgrade).to_string());
    form_state.vessel_upgrades[0] = new_vessel_1_upgrade;
    
    let vessel_2_upgrade_select = get_element_by_id!("vessel_2_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_vessel_2_upgrade = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[1].clone(), 1);
    vessel_2_upgrade_select.set_inner_html("");
    valid_vessel_upgrades[1].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_2_upgrade_select.append_child(&option).unwrap();});
    log_select!(vessel_2_upgrade_select, Into::<u8>::into(new_vessel_2_upgrade).to_string());
    form_state.vessel_upgrades[1] = new_vessel_2_upgrade;
    
    let vessel_3_upgrade_select = get_element_by_id!("vessel_3_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_vessel_3_upgrade = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[2].clone(), 2);
    vessel_3_upgrade_select.set_inner_html("");
    valid_vessel_upgrades[2].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_3_upgrade_select.append_child(&option).unwrap();});
    log_select!(vessel_3_upgrade_select, Into::<u8>::into(new_vessel_3_upgrade).to_string());
    form_state.vessel_upgrades[2] = new_vessel_3_upgrade;
    
    let vessel_4_upgrade_select = get_element_by_id!("vessel_4_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_vessel_4_upgrade = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[3].clone(), 3);
    vessel_4_upgrade_select.set_inner_html("");
    valid_vessel_upgrades[3].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_4_upgrade_select.append_child(&option).unwrap();});
    log_select!(vessel_4_upgrade_select, Into::<u8>::into(new_vessel_4_upgrade).to_string());
    form_state.vessel_upgrades[3] = new_vessel_4_upgrade;
    
    let vessel_5_upgrade_select = get_element_by_id!("vessel_5_upgrade").unchecked_into::<HtmlSelectElement>();
    let new_vessel_5_upgrade = data::VesselUpgrade::resolve_or_default_slot(&form_state, valid_vessel_upgrades[4].clone(), 4);
    vessel_5_upgrade_select.set_inner_html("");
    valid_vessel_upgrades[4].iter().map(|&v| Into::<HtmlOptionElement>::into(v)).for_each(|option| {vessel_5_upgrade_select.append_child(&option).unwrap();});
    log_select!(vessel_5_upgrade_select, Into::<u8>::into(new_vessel_5_upgrade).to_string());
    form_state.vessel_upgrades[4] = new_vessel_5_upgrade;

    let costs = form_state.costs();
    let base_cost = i32::from(form_state.unit_type.cost());
    let base_gold = i32::from(form_state.unit_type.unit_gold());
    let refit_cost = i32::from(form_state.refit_type.cost());
    let refit_gold = i32::from(form_state.refit_type.unit_gold());

    let a = allocate(&costs, base_gold, refit_gold, form_state.refit_type != data::RefitType::None);

    let item_total = costs.iter().sum::<i32>();
    let company_used = base_cost + refit_cost + (item_total - a.base_used - a.refit_used);
    let remaining_budget = i32::from(form_state.budget) - company_used;
    let unit_gold_remaining = (base_gold - a.base_used) + (refit_gold - a.refit_used);

    set_number_input("unit_gold", form_state.unit_type.unit_gold());
    set_number_input("refit_gold", form_state.refit_type.unit_gold());
    set_number_input("siege_cost", form_state.siege.cost());
    set_number_input("heavy_siege_cost", form_state.heavy_siege.cost());
    set_number_input("pack_cost", form_state.pack.cost());
    set_number_input("potion_cost", form_state.potion.cost());
    set_number_input("vessel_cost", form_state.vessel.cost());
    for (index, weapon) in form_state.weapons.iter().enumerate() {
        set_number_input(&format!("weapon_{}_cost", index + 1), weapon.cost());
    }
    for (index, upgrade) in form_state.weapon_upgrades.iter().enumerate() {
        set_number_input(&format!("weapon_{}_upgrade_cost", index + 1), upgrade.cost());
    }
    for (index, spell) in form_state.spells.iter().enumerate() {
        set_number_input(&format!("spell_{}_cost", index + 1), spell.cost());
    }
    for (index, structure) in form_state.structures.iter().enumerate() {
        set_number_input(&format!("structure_{}_cost", index + 1), structure.cost());
    }
    for (index, upgrade) in form_state.vessel_upgrades.iter().enumerate() {
        set_number_input(&format!("vessel_{}_upgrade_cost", index + 1), upgrade.cost());
    }

    set_image("unit_type_image", &form_state.unit_type.asset());
    set_image("refit_type_image", &form_state.refit_type.asset());
    set_image("siege_image", &form_state.siege.asset());
    set_image("heavy_siege_image", &form_state.heavy_siege.asset());
    set_image("pack_image", &form_state.pack.asset());
    set_image("potion_image", &form_state.potion.asset());
    set_image("vessel_image", &form_state.vessel.asset());
    for (index, weapon) in form_state.weapons.iter().enumerate() {
        set_image(&format!("weapon_{}_image", index + 1), &weapon.asset());
    }
    for (index, upgrade) in form_state.weapon_upgrades.iter().enumerate() {
        set_image(&format!("weapon_{}_upgrade_image", index + 1), &upgrade.asset());
    }
    for (index, spell) in form_state.spells.iter().enumerate() {
        set_image(&format!("spell_{}_image", index + 1), &spell.asset());
    }
    for (index, structure) in form_state.structures.iter().enumerate() {
        set_image(&format!("structure_{}_image", index + 1), &structure.asset());
    }
    for (index, upgrade) in form_state.vessel_upgrades.iter().enumerate() {
        set_image(&format!("vessel_{}_upgrade_image", index + 1), &upgrade.asset());
    }

    let remaining_budget_input = get_element_by_id!("remaining_budget").unchecked_into::<HtmlInputElement>();
    let unit_cost_input = get_element_by_id!("unit_cost").unchecked_into::<HtmlInputElement>();
    let unit_gold_remaining_input = get_element_by_id!("unit_gold_remaining").unchecked_into::<HtmlInputElement>();
    let refit_type_select = get_element_by_id!("refit_type").unchecked_into::<HtmlSelectElement>();

    remaining_budget_input.set_value(remaining_budget.to_string().as_str());
    unit_cost_input.set_value((base_cost + refit_cost + item_total).to_string().as_str());
    unit_gold_remaining_input.set_value(unit_gold_remaining.to_string().as_str());
    info!("budget {} company_used {} remaining {} unit_gold_remaining {}", form_state.budget, company_used, remaining_budget, unit_gold_remaining);
    debug!("allocation base_used {} refit_used {} refit_ok {}", a.base_used, a.refit_used, a.refit_ok);
    if remaining_budget < 0 {
        warn!("company cost {} exceeds budget {}", company_used, form_state.budget);
        remaining_budget_input.set_class_name("bad");
    } else {
        remaining_budget_input.set_class_name("");
    }
    if a.refit_ok {
        refit_type_select.set_class_name("");
    } else {
        warn!("refit gold cannot be spent exactly");
        refit_type_select.set_class_name("bad");
    }

    let code: [u8; 35] = form_state.into();
    let code_string = URL_SAFE_NO_PAD.encode(code);
    let copy_button = get_element_by_id!("copy_button").unchecked_into::<HtmlButtonElement>();
    copy_button.set_attribute("onclick",   format!(
        "navigator.clipboard.writeText(location.origin+location.pathname+'?L={}')",
        code_string
    ).as_str()).unwrap();
    debug!("copy button updated");

    if cfg!(debug_assertions) {
        let debug_text = get_element_by_id!("debug_text").unchecked_into::<HtmlParagraphElement>();
        debug_text.set_text_content(Some(format!("Code: {}", code_string).as_str()));
        debug!("debug text updated");
    }
}

fn set_number_input(id: &str, value: u8) {
    get_element_by_id!(id).unchecked_into::<HtmlInputElement>().set_value(&value.to_string());
}

fn set_image(id: &str, asset: &str) {
    get_element_by_id!(id).unchecked_into::<HtmlImageElement>().set_src(asset);
}

fn stored_save(storage: &web_sys::Storage, key: &str) -> String {
    match storage.get_item(key) {
        Ok(Some(value)) => {
            debug!("read {} ({} bytes)", key, value.len());
            value
        }
        Ok(None) => {
            debug!("{} is empty", key);
            String::new()
        }
        Err(err) => {
            warn!("failed to read {}: {:?}", key, err);
            String::new()
        }
    }
}

fn save(event: web_sys::Event) {
    let target = event.target().unwrap().unchecked_into::<HtmlButtonElement>();
    debug!("save clicked: {}", target.id());
    let localstorage = match web_sys::window().unwrap().local_storage() {
        Ok(Some(storage)) => storage,
        Ok(None) => {
            error!("cannot save, local storage is missing");
            return;
        }
        Err(err) => {
            error!("cannot save, local storage lookup failed: {:?}", err);
            return;
        }
    };
    let form = get_element_by_id!("form").unchecked_into::<HtmlFormElement>();
    let form_data = match web_sys::FormData::new_with_form(&form) {
        Ok(data) => data,
        Err(err) => {
            error!("cannot save, failed to read the form: {:?}", err);
            return;
        }
    };
    let form_state: FormState = match form_data.try_into() {
        Ok(state) => state,
        Err(field) => {
            error!("cannot save, invalid field {}", field);
            return;
        }
    };
    let code: [u8; 35] = form_state.into();
    let code_string = URL_SAFE_NO_PAD.encode(code);
    let key = format!("crusadercalc:{}", target.id());
    if let Err(err) = localstorage.set_item(&key, code_string.as_str()) {
        error!("failed to write {}: {:?}", key, err);
        return;
    }
    let load_input = get_element_by_id!( match target.id().as_str() {
        "save_1" => "load_1_input",
        "save_2" => "load_2_input",
        "save_3" => "load_3_input",
        "save_4" => "load_4_input",
        "save_5" => "load_5_input",
        other => {
            error!("saved {} but the button id {} has no load field", key, other);
            return;
        }
    }).unchecked_into::<HtmlInputElement>();
    load_input.set_value(code_string.as_str());
    info!("saved form to {} and updated {}", key, load_input.id());
}

pub struct Allocation {
    pub base_used: i32,
    pub refit_used: i32,
    pub refit_ok: bool, // false => mark refit input "bad"
}

pub fn allocate(costs: &[i32], base_gold: i32, refit_gold: i32, has_refit: bool) -> Allocation {
    debug!("allocate {} items, base_gold {}, refit_gold {}, has_refit {}", costs.len(), base_gold, refit_gold, has_refit);
    let refit_gold = if has_refit { refit_gold } else { 0 };
    let (bw, rw) = (base_gold as usize + 1, refit_gold as usize + 1);

    // ok[b * rw + r] = some assignment spends exactly b from base and r from refit
    let mut ok = vec![false; bw * rw];
    ok[0] = true;

    for &c in costs {
        let c = c as usize;
        // descending in both axes => in-place 0/1 knapsack (each item used once)
        for b in (0..bw).rev() {
            for r in (0..rw).rev() {
                let mut v = ok[b * rw + r];
                if b >= c { v |= ok[(b - c) * rw + r]; } // item -> base
                if r >= c { v |= ok[b * rw + (r - c)]; } // item -> refit
                ok[b * rw + r] = v;                       // else -> company
            }
        }
    }

    // Legal refit build: base fully spent; maximize refit usage.
    let b_full = bw - 1;
    if let Some(r) = (0..rw).rev().find(|&r| ok[b_full * rw + r]) {
        info!("base gold {} spent exactly, refit used {}", base_gold, r);
        return Allocation { base_used: b_full as i32, refit_used: r as i32, refit_ok: true };
    }

    // Base can't be spent exactly: refit pool locked, spend as much base as possible.
    let best_base = (0..bw).rev().find(|&b| ok[b * rw]).unwrap_or(0);
    warn!("base gold {} cannot be spent exactly, best base {}", base_gold, best_base);
    if has_refit {
        warn!("refit pool locked because base gold was not spent exactly");
    }
    Allocation { base_used: best_base as i32, refit_used: 0, refit_ok: !has_refit }
}