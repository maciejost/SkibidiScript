#![allow(nonstandard_style)]
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum JavaScriptKeyword {
    break_,
    case,
    catch,
    class,
    const_,
    default,
    delete,
    do_,
    else_,
    export,
    extends,
    false_,
    function,
    if_,
    import,
    in_,
    new,
    null,
    return_,
    switch,
    this,
    throw,
    true_,
    try_,
    as_,
    interface,
    let_,
    type_,
    undefined,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SkibidiScriptKeyword {
    scram,
    cook,
    findOut,
    iykyk,
    levelTenGyat,
    slay,
    exile,
    bruh,
    riddleMeThat,
    beGone,
    inflateMaxx,
    cap,
    skibidi,
    riddleMeThis,
    summon,
    plugged,
    crisp,
    brokeBehavior,
    hesDoneFor,
    letHim,
    that,
    throwItBack,
    noCap,
    fuckAround,
    twinn,
    squad,
    levelOneGyat,
    aura,
    wentOutToBuyCigarettes,
}

impl FromStr for SkibidiScriptKeyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "scram" => Ok(SkibidiScriptKeyword::scram),
            "cook" => Ok(SkibidiScriptKeyword::cook),
            "findOut" => Ok(SkibidiScriptKeyword::findOut),
            "iykyk" => Ok(SkibidiScriptKeyword::iykyk),
            "levelTenGyat" => Ok(SkibidiScriptKeyword::levelTenGyat),
            "slay" => Ok(SkibidiScriptKeyword::slay),
            "exile" => Ok(SkibidiScriptKeyword::exile),
            "bruh" => Ok(SkibidiScriptKeyword::bruh),
            "riddleMeThat" => Ok(SkibidiScriptKeyword::riddleMeThat),
            "beGone" => Ok(SkibidiScriptKeyword::beGone),
            "inflateMaxx" => Ok(SkibidiScriptKeyword::inflateMaxx),
            "cap" => Ok(SkibidiScriptKeyword::cap),
            "skibidi" => Ok(SkibidiScriptKeyword::skibidi),
            "riddleMeThis" => Ok(SkibidiScriptKeyword::riddleMeThis),
            "summon" => Ok(SkibidiScriptKeyword::summon),
            "plugged" => Ok(SkibidiScriptKeyword::plugged),
            "crisp" => Ok(SkibidiScriptKeyword::crisp),
            "brokeBehavior" => Ok(SkibidiScriptKeyword::brokeBehavior),
            "hesDoneFor" => Ok(SkibidiScriptKeyword::hesDoneFor),
            "letHim" => Ok(SkibidiScriptKeyword::letHim),
            "that" => Ok(SkibidiScriptKeyword::that),
            "throwItBack" => Ok(SkibidiScriptKeyword::throwItBack),
            "noCap" => Ok(SkibidiScriptKeyword::noCap),
            "fuckAround" => Ok(SkibidiScriptKeyword::fuckAround),
            "twinn" => Ok(SkibidiScriptKeyword::twinn),
            "squad" => Ok(SkibidiScriptKeyword::squad),
            "levelOneGyat" => Ok(SkibidiScriptKeyword::levelOneGyat),
            "aura" => Ok(SkibidiScriptKeyword::aura),
            "wentOutToBuyCigarettes" => Ok(SkibidiScriptKeyword::wentOutToBuyCigarettes),
            _ => Err(()),
        }
    }
}

fn create_keywords_map() -> HashMap<SkibidiScriptKeyword, JavaScriptKeyword> {
    let mut map = HashMap::new();
    map.insert(SkibidiScriptKeyword::scram, JavaScriptKeyword::break_);
    map.insert(SkibidiScriptKeyword::cook, JavaScriptKeyword::case);
    map.insert(SkibidiScriptKeyword::findOut, JavaScriptKeyword::catch);
    map.insert(SkibidiScriptKeyword::iykyk, JavaScriptKeyword::class);
    map.insert(
        SkibidiScriptKeyword::levelTenGyat,
        JavaScriptKeyword::const_,
    );
    map.insert(SkibidiScriptKeyword::slay, JavaScriptKeyword::default);
    map.insert(SkibidiScriptKeyword::exile, JavaScriptKeyword::delete);
    map.insert(SkibidiScriptKeyword::bruh, JavaScriptKeyword::do_);
    map.insert(SkibidiScriptKeyword::riddleMeThat, JavaScriptKeyword::else_);
    map.insert(SkibidiScriptKeyword::beGone, JavaScriptKeyword::export);
    map.insert(
        SkibidiScriptKeyword::inflateMaxx,
        JavaScriptKeyword::extends,
    );
    map.insert(SkibidiScriptKeyword::cap, JavaScriptKeyword::false_);
    map.insert(SkibidiScriptKeyword::skibidi, JavaScriptKeyword::function);
    map.insert(SkibidiScriptKeyword::riddleMeThis, JavaScriptKeyword::if_);
    map.insert(SkibidiScriptKeyword::summon, JavaScriptKeyword::import);
    map.insert(SkibidiScriptKeyword::plugged, JavaScriptKeyword::in_);
    map.insert(SkibidiScriptKeyword::crisp, JavaScriptKeyword::new);
    map.insert(SkibidiScriptKeyword::brokeBehavior, JavaScriptKeyword::null);
    map.insert(SkibidiScriptKeyword::hesDoneFor, JavaScriptKeyword::return_);
    map.insert(SkibidiScriptKeyword::letHim, JavaScriptKeyword::switch);
    map.insert(SkibidiScriptKeyword::that, JavaScriptKeyword::this);
    map.insert(SkibidiScriptKeyword::throwItBack, JavaScriptKeyword::throw);
    map.insert(SkibidiScriptKeyword::noCap, JavaScriptKeyword::true_);
    map.insert(SkibidiScriptKeyword::fuckAround, JavaScriptKeyword::try_);
    map.insert(SkibidiScriptKeyword::twinn, JavaScriptKeyword::as_);
    map.insert(SkibidiScriptKeyword::squad, JavaScriptKeyword::interface);
    map.insert(SkibidiScriptKeyword::levelOneGyat, JavaScriptKeyword::let_);
    map.insert(SkibidiScriptKeyword::aura, JavaScriptKeyword::type_);
    map.insert(
        SkibidiScriptKeyword::wentOutToBuyCigarettes,
        JavaScriptKeyword::undefined,
    );
    map
}

pub fn js_keyword(skibidi_keyword_str: &str) -> String {
    let keywords_map = create_keywords_map();
    match SkibidiScriptKeyword::from_str(skibidi_keyword_str) {
        Ok(skibidi_keyword) => {
            let js_keyword = keywords_map.get(&skibidi_keyword).unwrap();
            format!("{:?}", js_keyword).replace("_", "")
        }
        Err(_) => skibidi_keyword_str.to_string(),
    }
}
