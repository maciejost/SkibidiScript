#[derive(Debug)]
pub enum JavaScriptKeyword {
    break_,
    case,
    catch,
    class,
    const_,
    continue_,
    debugger,
    default,
    delete,
    do_,
    else_,
    enum_,
    export,
    extends,
    false_,
    finally,
    for_,
    function,
    if_,
    import,
    in_,
    instanceof,
    new,
    null,
    return_,
    super_,
    switch,
    this,
    throw,
    true_,
    try_,
    typeof_,
    var,
    as_,
    implements,
    interface,
    let_,
    package,
    private,
    protected,
    public,
    static_,
    yield_,
    void,
    while_,
    with,
    type_,
    undefined,
}

#[derive(Debug)]
pub enum SkibidiScriptKeyword {
    scram,
    cook,
    findOut,
    iykyk,
    levelTenGyat,
    continue_,
    debugger,
    slay,
    exile,
    bruh,
    riddleMeThat,
    enum_,
    beGone,
    inflateMaxx,
    cap,
    finally,
    for_,
    skibidi,
    riddleMeThis,
    summon,
    plugged,
    instanceOf,
    crisp,
    brokeBehavior,
    hesDoneFor,
    super_,
    letHim,
    that,
    throwItBack,
    noCap,
    fuckAround,
    typeof_,
    var,
    twinn,
    implements,
    squad,
    levelOneGyat,
    package,
    private,
    protected,
    public,
    static_,
    yield_,
    void,
    while_,
    with,
    aura,
    wentOutToBuyCigarettes,
}

pub const KEYWORDS: [(JavaScriptKeyword, SkibidiScriptKeyword); 48] = [
    (JavaScriptKeyword::break_, SkibidiScriptKeyword::scram),
    (JavaScriptKeyword::case, SkibidiScriptKeyword::cook),
    (JavaScriptKeyword::catch, SkibidiScriptKeyword::findOut),
    (JavaScriptKeyword::class, SkibidiScriptKeyword::iykyk),
    (
        JavaScriptKeyword::const_,
        SkibidiScriptKeyword::levelTenGyat,
    ),
    (
        JavaScriptKeyword::continue_,
        SkibidiScriptKeyword::continue_,
    ),
    (JavaScriptKeyword::debugger, SkibidiScriptKeyword::debugger),
    (JavaScriptKeyword::default, SkibidiScriptKeyword::slay),
    (JavaScriptKeyword::delete, SkibidiScriptKeyword::exile),
    (JavaScriptKeyword::do_, SkibidiScriptKeyword::bruh),
    (JavaScriptKeyword::else_, SkibidiScriptKeyword::riddleMeThat),
    (JavaScriptKeyword::enum_, SkibidiScriptKeyword::enum_),
    (JavaScriptKeyword::export, SkibidiScriptKeyword::beGone),
    (
        JavaScriptKeyword::extends,
        SkibidiScriptKeyword::inflateMaxx,
    ),
    (JavaScriptKeyword::false_, SkibidiScriptKeyword::cap),
    (JavaScriptKeyword::finally, SkibidiScriptKeyword::finally),
    (JavaScriptKeyword::for_, SkibidiScriptKeyword::for_),
    (JavaScriptKeyword::function, SkibidiScriptKeyword::skibidi),
    (JavaScriptKeyword::if_, SkibidiScriptKeyword::riddleMeThis),
    (JavaScriptKeyword::import, SkibidiScriptKeyword::summon),
    (JavaScriptKeyword::in_, SkibidiScriptKeyword::plugged),
    (
        JavaScriptKeyword::instanceof,
        SkibidiScriptKeyword::instanceOf,
    ),
    (JavaScriptKeyword::new, SkibidiScriptKeyword::crisp),
    (JavaScriptKeyword::null, SkibidiScriptKeyword::brokeBehavior),
    (JavaScriptKeyword::return_, SkibidiScriptKeyword::hesDoneFor),
    (JavaScriptKeyword::super_, SkibidiScriptKeyword::super_),
    (JavaScriptKeyword::switch, SkibidiScriptKeyword::letHim),
    (JavaScriptKeyword::this, SkibidiScriptKeyword::that),
    (JavaScriptKeyword::throw, SkibidiScriptKeyword::throwItBack),
    (JavaScriptKeyword::true_, SkibidiScriptKeyword::noCap),
    (JavaScriptKeyword::try_, SkibidiScriptKeyword::fuckAround),
    (JavaScriptKeyword::typeof_, SkibidiScriptKeyword::typeof_),
    (JavaScriptKeyword::var, SkibidiScriptKeyword::var),
    (JavaScriptKeyword::as_, SkibidiScriptKeyword::twinn),
    (
        JavaScriptKeyword::implements,
        SkibidiScriptKeyword::implements,
    ),
    (JavaScriptKeyword::interface, SkibidiScriptKeyword::squad),
    (JavaScriptKeyword::let_, SkibidiScriptKeyword::levelOneGyat),
    (JavaScriptKeyword::package, SkibidiScriptKeyword::package),
    (JavaScriptKeyword::private, SkibidiScriptKeyword::private),
    (
        JavaScriptKeyword::protected,
        SkibidiScriptKeyword::protected,
    ),
    (JavaScriptKeyword::public, SkibidiScriptKeyword::public),
    (JavaScriptKeyword::static_, SkibidiScriptKeyword::static_),
    (JavaScriptKeyword::yield_, SkibidiScriptKeyword::yield_),
    (JavaScriptKeyword::void, SkibidiScriptKeyword::void),
    (JavaScriptKeyword::while_, SkibidiScriptKeyword::while_),
    (JavaScriptKeyword::with, SkibidiScriptKeyword::with),
    (JavaScriptKeyword::type_, SkibidiScriptKeyword::aura),
    (
        JavaScriptKeyword::undefined,
        SkibidiScriptKeyword::wentOutToBuyCigarettes,
    ),
];
