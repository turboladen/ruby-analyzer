use crate::node::Loc;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Alias {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct And {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AndAsgn {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Arg {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Args {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Array {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ArrayPattern {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ArrayPatternWithTail {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BackRef {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Begin {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Block {
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct BlockPass {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Blockarg {
    pub(crate) name: Option<String>,

    pub(crate) operator_l: Loc,
    pub(crate) name_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Break {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CSend {
    pub(crate) method_name: String,

    pub(crate) dot_l: Loc,
    pub(crate) selector_l: Option<Loc>,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Case {
    pub(crate) keyword_l: Loc,
    pub(crate) else_l: Option<Loc>,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CaseMatch {
    pub(crate) keyword_l: Loc,
    pub(crate) else_l: Option<Loc>,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Casgn {
    pub(crate) name: String,
    pub(crate) double_colon_l: Option<Loc>,
    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Option<Loc>,
    pub(crate) end_l: Loc,
}

impl Class {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn keyword_l(&self) -> Loc {
        self.keyword_l
    }

    pub fn operator_l(&self) -> Option<Loc> {
        self.operator_l
    }

    pub fn end_l(&self) -> Loc {
        self.end_l
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Complex {
    pub(crate) value: String,

    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Const {
    pub(crate) name: String,
    pub(crate) double_colon_l: Option<Loc>,
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ConstPattern {
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Cvar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Cvasgn {
    pub(crate) name: String,
    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Def {
    pub(crate) name: String,

    pub(crate) keyword_l: Loc,
    pub(crate) name_l: Loc,
    pub(crate) end_l: Option<Loc>,
    pub(crate) assignment_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Defined {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Defs {
    pub(crate) name: String,

    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Loc,
    pub(crate) name_l: Loc,
    pub(crate) assignment_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Dstr {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Dsym {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EFlipFlop {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Ensure {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Erange {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FindPattern {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Float {
    pub(crate) value: String,

    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct For {
    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Gvar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Gvasgn {
    pub(crate) name: String,
    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Hash {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HashPattern {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Heredoc {
    pub(crate) heredoc_body_l: Loc,
    pub(crate) heredoc_end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IFlipFlop {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct If {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) else_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IfGuard {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IfMod {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IfTernary {
    pub(crate) question_l: Loc,
    pub(crate) colon_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InPattern {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Index {
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IndexAsgn {
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Int {
    pub(crate) value: String,

    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Irange {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Ivar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Ivasgn {
    pub(crate) name: String,
    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct KwBegin {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Kwarg {
    pub(crate) name: String,

    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Kwnilarg {
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Kwoptarg {
    pub(crate) name: String,
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Kwrestarg {
    pub(crate) name: Option<String>,

    pub(crate) operator_l: Loc,
    pub(crate) name_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Kwsplat {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Lvar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Lvasgn {
    pub(crate) name: String,
    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Masgn {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchAlt {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchAs {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchNilPattern {
    pub(crate) operator_l: Loc,
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchPattern {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchPatternP {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MatchRest {
    pub(crate) name: Option<String>,
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MatchVar {
    pub(crate) name: String,
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchWithLvasgn {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Mlhs {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Module {
    pub(crate) name: String,
    pub(crate) keyword_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Next {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct NthRef {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Numblock {
    pub(crate) numargs: u8,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct OpAsgn {
    pub(crate) operator: String,
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Optarg {
    pub(crate) name: String,
    pub(crate) name_l: Loc,
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Or {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct OrAsgn {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pair {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pin {
    pub(crate) selector_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Postexe {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Preexe {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Procarg0 {
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Rational {
    pub(crate) value: String,

    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RegOpt {
    pub(crate) options: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Regexp {
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Rescue {
    pub(crate) else_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct RescueBody {
    pub(crate) keyword_l: Loc,
    pub(crate) assoc_l: Option<Loc>,
    pub(crate) begin_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Restarg {
    pub(crate) name: Option<String>,

    pub(crate) operator_l: Loc,
    pub(crate) name_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Return {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SClass {
    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Send {
    pub(crate) method_name: String,
    pub(crate) dot_l: Option<Loc>,
    pub(crate) selector_l: Option<Loc>,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Shadowarg {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Splat {
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Str {
    pub(crate) value: Vec<u8>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Super {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Sym {
    pub(crate) name: String,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Undef {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct UnlessGuard {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Until {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct UntilPost {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct When {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct While {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct WhilePost {
    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct XHeredoc {
    pub(crate) heredoc_body_l: Loc,
    pub(crate) heredoc_end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Xstr {
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Yield {
    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}
