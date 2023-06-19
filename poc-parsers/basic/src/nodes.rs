use crate::node::Loc;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Alias {
    pub(crate) to_id: usize,
    pub(crate) from_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct And {
    pub(crate) lhs_id: usize,
    pub(crate) rhs_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AndAsgn {
    pub(crate) recv_id: usize,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Arg {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Args {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Array {
    pub(crate) element_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ArrayPattern {
    pub(crate) element_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ArrayPatternWithTail {
    pub(crate) element_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BackRef {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Begin {
    pub(crate) statement_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Block {
    pub(crate) call_id: usize,
    pub(crate) args_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct BlockPass {
    pub(crate) value_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Blockarg {
    pub(crate) name: Option<String>,

    pub(crate) operator_l: Loc,
    pub(crate) name_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Break {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CSend {
    pub(crate) recv_id: usize,
    pub(crate) method_name: String,

    pub(crate) arg_ids: Vec<usize>,

    pub(crate) dot_l: Loc,
    pub(crate) selector_l: Option<Loc>,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Case {
    pub(crate) expr_id: Option<usize>,
    pub(crate) when_body_ids: Vec<usize>,
    pub(crate) else_body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) else_l: Option<Loc>,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CaseMatch {
    pub(crate) expr_id: usize,
    pub(crate) in_body_ids: Vec<usize>,
    pub(crate) else_body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) else_l: Option<Loc>,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Casgn {
    pub(crate) name: String,
    pub(crate) scope_id: Option<usize>,
    pub(crate) value_id: Option<usize>,

    pub(crate) double_colon_l: Option<Loc>,
    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Cbase;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) name_id: usize,
    pub(crate) superclass_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Option<Loc>,
    pub(crate) end_l: Loc,
}

impl Class {
    pub fn name(&self) -> &str {
        self.name.as_ref()
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
    pub(crate) scope_id: Option<usize>,

    pub(crate) double_colon_l: Option<Loc>,
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ConstPattern {
    pub(crate) const_id: usize,
    pub(crate) pattern_id: usize,

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
    pub(crate) value_id: Option<usize>,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Def {
    pub(crate) name: String,

    pub(crate) args_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) name_l: Loc,
    pub(crate) end_l: Option<Loc>,
    pub(crate) assignment_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Defined {
    pub(crate) value_id: usize,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Defs {
    pub(crate) definee_id: usize,
    pub(crate) name: String,

    pub(crate) args_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Loc,
    pub(crate) name_l: Loc,
    pub(crate) assignment_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Dstr {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Dsym {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EFlipFlop {
    pub(crate) left_id: Option<usize>,
    pub(crate) right_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EmptyElse;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Encoding;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Ensure {
    pub(crate) body_id: Option<usize>,
    pub(crate) ensure_id: Option<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Erange {
    pub(crate) left_id: Option<usize>,
    pub(crate) right_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct False;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct File;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FindPattern {
    pub(crate) element_ids: Vec<usize>,

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
    pub(crate) iterator_id: usize,
    pub(crate) iteratee_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ForwardArg;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ForwardedArgs;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Gvar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Gvasgn {
    pub(crate) name: String,
    pub(crate) value_id: Option<usize>,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Hash {
    pub(crate) pair_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HashPattern {
    pub(crate) element_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Heredoc {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) heredoc_body_l: Loc,
    pub(crate) heredoc_end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IFlipFlop {
    pub(crate) left_id: Option<usize>,
    pub(crate) right_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct If {
    pub(crate) cond_id: usize,
    pub(crate) if_true_id: Option<usize>,
    pub(crate) if_false_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) else_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IfGuard {
    pub(crate) cond_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IfMod {
    pub(crate) cond_id: usize,
    pub(crate) if_true_id: Option<usize>,
    pub(crate) if_false_id: Option<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IfTernary {
    pub(crate) cond_id: usize,
    pub(crate) if_true_id: usize,
    pub(crate) if_false_id: usize,

    pub(crate) question_l: Loc,
    pub(crate) colon_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InPattern {
    pub(crate) pattern_id: usize,
    pub(crate) guard_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Index {
    pub(crate) recv_id: usize,
    pub(crate) index_ids: Vec<usize>,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct IndexAsgn {
    pub(crate) recv_id: usize,
    pub(crate) index_ids: Vec<usize>,
    pub(crate) value_id: Option<usize>,

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
    pub(crate) left_id: Option<usize>,
    pub(crate) right_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Ivar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Ivasgn {
    pub(crate) name: String,
    pub(crate) value_id: Option<usize>,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct KwBegin {
    pub(crate) statement_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Kwarg {
    pub(crate) name: String,

    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Kwargs {
    pub(crate) pair_ids: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Kwnilarg {
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Kwoptarg {
    pub(crate) name: String,
    pub(crate) default_id: usize,

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
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Lambda;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Line;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Lvar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Lvasgn {
    pub(crate) name: String,
    pub(crate) value_id: Option<usize>,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Masgn {
    pub(crate) lhs_id: usize,
    pub(crate) rhs_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchAlt {
    pub(crate) lhs_id: usize,
    pub(crate) rhs_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchAs {
    pub(crate) value_id: usize,
    pub(crate) as_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchCurrentLine {
    pub(crate) re_id: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchNilPattern {
    pub(crate) operator_l: Loc,
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchPattern {
    pub(crate) value_id: usize,
    pub(crate) pattern_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchPatternP {
    pub(crate) value_id: usize,
    pub(crate) pattern_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MatchRest {
    pub(crate) name: Option<String>,
    pub(crate) name_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MatchVar {
    pub(crate) name: String,

    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MatchWithLvasgn {
    pub(crate) re_id: usize,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Mlhs {
    pub(crate) item_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Module {
    pub(crate) name: String,
    pub(crate) name_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Next {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Nil;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct NthRef {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Numblock {
    pub(crate) call_id: usize,
    pub(crate) numargs: u8,
    pub(crate) body_id: usize,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct OpAsgn {
    pub(crate) recv_id: usize,
    pub(crate) operator: String,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Optarg {
    pub(crate) name: String,
    pub(crate) default_id: usize,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Or {
    pub(crate) lhs_id: usize,
    pub(crate) rhs_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct OrAsgn {
    pub(crate) recv_id: usize,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pair {
    pub(crate) key_id: usize,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pin {
    pub(crate) var_id: usize,

    pub(crate) selector_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Postexe {
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Preexe {
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Procarg0 {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Rational {
    pub(crate) value: String,

    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Redo;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RegOpt {
    pub(crate) options: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Regexp {
    pub(crate) part_ids: Vec<usize>,
    pub(crate) options_id: Option<usize>,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Rescue {
    pub(crate) body_id: Option<usize>,
    pub(crate) rescue_body_ids: Vec<usize>,
    pub(crate) else_id: Option<usize>,

    pub(crate) else_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct RescueBody {
    pub(crate) exc_list_id: Option<usize>,
    pub(crate) exc_var_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

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
pub struct Retry;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Return {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SClass {
    pub(crate) expr_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Self_;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Send {
    pub(crate) method_name: String,
    pub(crate) recv_id: Option<usize>,
    pub(crate) arg_ids: Vec<usize>,

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
    pub(crate) value_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Str {
    pub(crate) value: Vec<u8>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Super {
    pub(crate) arg_ids: Vec<usize>,

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
pub struct True;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Undef {
    pub(crate) name_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct UnlessGuard {
    pub(crate) cond_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Until {
    pub(crate) cond_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct UntilPost {
    pub(crate) cond_id: usize,
    pub(crate) body_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct When {
    pub(crate) pattern_ids: Vec<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct While {
    pub(crate) cond_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct WhilePost {
    pub(crate) cond_id: usize,
    pub(crate) body_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct XHeredoc {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) heredoc_body_l: Loc,
    pub(crate) heredoc_end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Xstr {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Yield {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ZSuper;
