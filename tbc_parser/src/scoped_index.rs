pub mod nodes;

use std::collections::BTreeMap;

use self::nodes::*;
use crate::{location::Loc, ScopeGate};

// pub type ScopedIndex = BTreeMap<ScopeGateNode, ScopeItems>;

// #[derive(Debug, Default)]
// pub struct ScopeItems {
//     child_scopes: ScopedIndex,
//     nodes: Vec<Node>,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedIndex {
    inner: BTreeMap<ScopeGate, Vec<Node>>,
}

impl Default for ScopedIndex {
    fn default() -> Self {
        Self {
            inner: {
                let mut map = BTreeMap::default();
                map.insert(ScopeGate::default(), vec![]);
                map
            },
        }
    }
}

impl ScopedIndex {
    pub fn inner_mut(&mut self) -> &mut BTreeMap<ScopeGate, Vec<Node>> {
        &mut self.inner
    }

    pub fn inner(&self) -> &BTreeMap<ScopeGate, Vec<Node>> {
        &self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub(crate) id: usize,
    pub(crate) properties: NodeProperties,
}

impl Node {
    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeProperties {
    Alias(Alias),
    And(And),
    AndAsgn(AndAsgn),
    Arg(Arg),
    Args(Args),
    Array(Array),
    ArrayPattern(ArrayPattern),
    ArrayPatternWithTail(ArrayPatternWithTail),
    BackRef(BackRef),
    Begin(Begin),
    Block(Block),
    BlockPass(BlockPass),
    Blockarg(Blockarg),
    Break(Break),
    Case(Case),
    CaseMatch(CaseMatch),
    Casgn(Casgn),
    Cbase,
    Class(Class),
    Complex(Complex),
    Const(Const),
    ConstPattern(ConstPattern),
    CSend(CSend),
    Cvar(Cvar),
    Cvasgn(Cvasgn),
    Def(Def),
    Defined(Defined),
    Defs(Defs),
    Dstr(Dstr),
    Dsym(Dsym),
    EFlipFlop(EFlipFlop),
    EmptyElse,
    Encoding,
    Ensure(Ensure),
    Erange(Erange),
    False,
    File,
    FindPattern(FindPattern),
    Float(Float),
    For(For),
    ForwardArg,
    ForwardedArgs,
    Gvar(Gvar),
    Gvasgn(Gvasgn),
    Hash(Hash),
    HashPattern(HashPattern),
    Heredoc(Heredoc), // TODO: completion
    IFlipFlop(IFlipFlop),
    If(If),
    IfGuard(IfGuard),
    IfMod(IfMod),
    IfTernary(IfTernary),
    InPattern(InPattern),
    Index(Index),
    IndexAsgn(IndexAsgn),
    Int(Int),
    Irange(Irange),
    Ivar(Ivar),
    Ivasgn(Ivasgn),
    KwBegin(KwBegin),
    Kwarg(Kwarg),
    Kwargs(Kwargs),
    Kwnilarg,
    Kwoptarg(Kwoptarg),
    Kwrestarg(Kwrestarg),
    Kwsplat(Kwsplat),
    Lambda,
    Line,
    Lvar(Lvar),
    Lvasgn(Lvasgn),
    Masgn(Masgn),
    MatchAlt(MatchAlt),
    MatchAs(MatchAs),
    MatchCurrentLine(MatchCurrentLine),
    MatchNilPattern,
    MatchPattern(MatchPattern),   // TODO: completion?
    MatchPatternP(MatchPatternP), // TODO: completion?
    MatchRest(MatchRest),
    MatchVar(MatchVar),
    MatchWithLvasgn(MatchWithLvasgn), // TODO: completion?
    Mlhs(Mlhs),
    Module(Module),
    Next(Next),
    Nil,
    NthRef(NthRef),
    Numblock(Numblock), // TODO: completion?
    OpAsgn(OpAsgn),
    Optarg(Optarg),
    Or(Or),
    OrAsgn(OrAsgn),
    Pair(Pair),
    Pin(Pin),
    Postexe(Postexe),
    Preexe(Preexe),
    Procarg0(Procarg0),
    Rational(Rational),
    Redo,
    RegOpt(RegOpt),
    Regexp(Regexp),
    Rescue(Rescue),
    RescueBody(RescueBody),
    Restarg(Restarg),
    Retry,
    Return(Return),
    SClass(SClass),
    Self_,
    Send(Send),
    Shadowarg(Shadowarg),
    Splat(Splat),
    Str(Str),
    Super(Super),
    Sym(Sym),
    True,
    Undef(Undef),
    UnlessGuard(UnlessGuard),
    Until(Until),
    UntilPost(UntilPost),
    When(When),
    While(While),
    WhilePost(WhilePost),
    XHeredoc(XHeredoc),
    Xstr(Xstr),
    Yield(Yield),
    ZSuper,
}
