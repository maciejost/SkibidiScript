#[derive(Debug)]
enum JavaScriptKeyword {
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
}

#[derive(Debug)]
enum SkibidiScriptKeyword {
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
}

fn main() {
    // Example usage
    let keyword = JavaScriptKeyword::const_;
    let skibidiKeyword = SkibidiScriptKeyword::levelTenGyat;
    println!("{:?}", keyword,);
    println!("{:?}", skibidiKeyword,);
}
