#[macro_use]
pub mod macros;


pub struct PIESHeader {
    pies_version: String,
    sub_type: String,
    blanket_effect_date: String,
    changes_since_date: String,
    parent_duns_number: String,
    parent_gln: String,
    parent_vmrsid: String,
    parent_aaiaid: String,
    brand_owner_duns: String,
    brand_owner_gln: String,
    brand_owner_vmrsid: String,
    brand_owner_aaiaid: String,
    buyer_duns: String,
    currency_code: String,
    lang_code: String,
    tech_contact: String,
    contact_email: String,
    pcdb_ver_date: String,
    padb_ver_date: String
}

pub struct PriceSheets {
    sheets: Vec<PriceSheet>,
}

pub struct PriceSheet {
    price_sheet: XmlContent,
    price_sheet_name: String,
    super_price_sheet_number: String,
    currency_code: String,
    price_zone: String,
    effective_date: String,
    exp_date: String,
}

pub struct MarketCopy {
    market_copy_cont: XmlContent,
    digital_assets: Vec<DigitalFileInfo>,
}

pub struct Product {
    item: XmlCouple,
    hazard_mat_code: String,
    base_item_id: String,
    item_lvl_gtin: XmlContent,
    part_number: String,
    brand_aaiaid: String,
    brand_label: String,
    vmrs_brand_id: String,
    qty_per_app: XmlContent,
    item_effective_date: String,
    avail_date: String,
    min_order_qty: XmlContent,
    mfg_prod_codes: XmlContent,
    aaia_prod_cat_code: String,
    unspsc: String,
    part_terminology_id: String,
    vmrs_code: String,
    descriptions: Vec<XmlContent>,
    price: Vec<Price>,
    expi: Vec<XmlContent>,
    attributes: Vec<XmlContent>,
    package: Vec<Package>,
    kits: Vec<Kits>,
    interchage: Vec<PartInterchange>,
    digital_assets: Vec<DigitalFileInfo>,
}

pub struct Trailer {
    item_count: String,
    trans_date: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct XmlContent {
    attributes: Vec<XmlCouple>,
    value: String,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct XmlCouple {
    name: String,
    value: String,
}

pub struct PartInterchange {
    interchange_info: XmlContent,
    part_number: XmlContent,
}

pub struct DigitalFileInfo {
    file_name: String,
    asset_type: String,
    file_type: String,
    representation: String,
    file_size: String,
    resolution: String,
    color_mode: String,
    background: String,
    orientation_view: String,
    asset_dimension: XmlContent,
    asset_dates: XmlContent,
    country: String,
}
pub struct Kits {
    comp_part_number: XmlContent,
    comp_brand: String,
    comp_brand_label: String,
    comp_sub_brand: String,
    com_sub_brand_label: String,
    description: XmlContent,
    qty_in_kit: XmlContent,
    seq_code: String,
    sold_sep: String,
}
pub struct Price {
    pricing: XmlContent,
    price_sheet_number: String,
    currency_code: String,
    effective_date: String,
    exp_date: String,
    price: XmlContent,
    price_break: XmlContent,
}

pub struct ConsumerPackage {
    pack: XmlContent,
    pack_lvl_gtin: String,
    pack_bar_code_chars: String,
    pack_uom: String,
    qty_of_eaches: String,
    dimensions: PackDimensions,
    weights: PackWeights,
    weight_variance: String,
}

pub struct PackDimensions {
    merch_h: String,
    merch_w: String,
    merch_l: String,
    ship_h: String,
    ship_w: String,
    ship_l: String,
}

pub struct PackWeights {
    weight: String,
    dim_weight: String,
}

pub enum Package {
    ConsumerPackage,
    RegulatedPackage,
}
