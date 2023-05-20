use lib_ruby_parser::{Bytes, Loc};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Alias {
    pub(crate) to_id: usize,
    pub(crate) from_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct And {
    pub(crate) lhs_id: usize,
    pub(crate) rhs_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AndAsgn {
    pub(crate) recv_id: usize,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Arg {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Args {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Array {
    pub(crate) element_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayPattern {
    pub(crate) element_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayPatternWithTail {
    pub(crate) element_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BackRef {
    pub(crate) name: String,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Begin {
    pub(crate) statement_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Block {
    pub(crate) call_id: usize,
    pub(crate) args_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BlockPass {
    pub(crate) value_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Blockarg {
    pub(crate) name: Option<String>,

    pub(crate) operator_l: Loc,
    pub(crate) name_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Break {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Case {
    pub(crate) expr_id: Option<usize>,
    pub(crate) when_body_ids: Vec<usize>,
    pub(crate) else_body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) else_l: Option<Loc>,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CaseMatch {
    pub(crate) expr_id: usize,
    pub(crate) in_body_ids: Vec<usize>,
    pub(crate) else_body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) else_l: Option<Loc>,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Casgn {
    pub(crate) name: String,
    pub(crate) scope_id: Option<usize>,
    pub(crate) value_id: Option<usize>,

    pub(crate) double_colon_l: Option<Loc>,
    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cbase;

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Complex {
    pub(crate) value: String,

    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Const {
    pub(crate) name: String,
    pub(crate) scope_id: Option<usize>,

    pub(crate) double_colon_l: Option<Loc>,
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConstPattern {
    pub(crate) const_id: usize,
    pub(crate) pattern_id: usize,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cvar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cvasgn {
    pub(crate) name: String,
    pub(crate) value_id: Option<usize>,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Def {
    pub(crate) name: String,

    pub(crate) args_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) name_l: Loc,
    pub(crate) end_l: Option<Loc>,
    pub(crate) assignment_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Defined {
    pub(crate) value_id: usize,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dstr {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dsym {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EFlipFlop {
    pub(crate) left_id: Option<usize>,
    pub(crate) right_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EmptyElse;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Encoding;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Ensure {
    pub(crate) body_id: Option<usize>,
    pub(crate) ensure_id: Option<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Erange {
    pub(crate) left_id: Option<usize>,
    pub(crate) right_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct False;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct File;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FindPattern {
    pub(crate) element_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Float {
    pub(crate) value: String,

    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct For {
    pub(crate) iterator_id: usize,
    pub(crate) iteratee_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ForwardArg;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ForwardedArgs;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Gvar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Gvasgn {
    pub(crate) name: String,
    pub(crate) value_id: Option<usize>,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hash {
    pub(crate) pair_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HashPattern {
    pub(crate) element_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Heredoc {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) heredoc_body_l: Loc,
    pub(crate) heredoc_end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IFlipFlop {
    pub(crate) left_id: Option<usize>,
    pub(crate) right_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct If {
    pub(crate) cond_id: usize,
    pub(crate) if_true_id: Option<usize>,
    pub(crate) if_false_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) else_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IfGuard {
    pub(crate) cond_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IfMod {
    pub(crate) cond_id: usize,
    pub(crate) if_true_id: Option<usize>,
    pub(crate) if_false_id: Option<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IfTernary {
    pub(crate) cond_id: usize,
    pub(crate) if_true_id: usize,
    pub(crate) if_false_id: usize,

    pub(crate) question_l: Loc,
    pub(crate) colon_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InPattern {
    pub(crate) pattern_id: usize,
    pub(crate) guard_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Index {
    pub(crate) recv_id: usize,
    pub(crate) index_ids: Vec<usize>,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IndexAsgn {
    pub(crate) recv_id: usize,
    pub(crate) index_ids: Vec<usize>,
    pub(crate) value_id: Option<usize>,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Int {
    pub(crate) value: String,

    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Irange {
    pub(crate) left_id: Option<usize>,
    pub(crate) right_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ivar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ivasgn {
    pub(crate) name: String,
    pub(crate) value_id: Option<usize>,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KwBegin {
    pub(crate) statement_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Kwarg {
    pub(crate) name: String,

    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Kwargs {
    pub(crate) pair_ids: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Kwnilarg {
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Kwoptarg {
    pub(crate) name: String,
    pub(crate) default_id: usize,

    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Kwrestarg {
    pub(crate) name: Option<String>,

    pub(crate) operator_l: Loc,
    pub(crate) name_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Kwsplat {
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Lambda;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Line;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lvar {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lvasgn {
    pub(crate) name: String,
    pub(crate) value_id: Option<usize>,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Masgn {
    pub(crate) lhs_id: usize,
    pub(crate) rhs_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MatchAlt {
    pub(crate) lhs_id: usize,
    pub(crate) rhs_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MatchAs {
    pub(crate) value_id: usize,
    pub(crate) as_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MatchCurrentLine {
    pub(crate) re_id: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MatchNilPattern {
    pub(crate) operator_l: Loc,
    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MatchPattern {
    pub(crate) value_id: usize,
    pub(crate) pattern_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MatchPatternP {
    pub(crate) value_id: usize,
    pub(crate) pattern_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MatchRest {
    pub(crate) name: Option<String>,
    pub(crate) name_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MatchVar {
    pub(crate) name: String,

    pub(crate) name_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MatchWithLvasgn {
    pub(crate) re_id: usize,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Mlhs {
    pub(crate) item_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Module {
    pub(crate) name: String,
    pub(crate) name_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Next {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Nil;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NthRef {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Numblock {
    pub(crate) call_id: usize,
    pub(crate) numargs: u8,
    pub(crate) body_id: usize,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OpAsgn {
    pub(crate) recv_id: usize,
    pub(crate) operator: String,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Optarg {
    pub(crate) name: String,
    pub(crate) default_id: usize,

    pub(crate) name_l: Loc,
    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Or {
    pub(crate) lhs_id: usize,
    pub(crate) rhs_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OrAsgn {
    pub(crate) recv_id: usize,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pair {
    pub(crate) key_id: usize,
    pub(crate) value_id: usize,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pin {
    pub(crate) var_id: usize,

    pub(crate) selector_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Postexe {
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Preexe {
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Procarg0 {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rational {
    pub(crate) value: String,

    pub(crate) operator_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Redo;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RegOpt {
    pub(crate) options: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Regexp {
    pub(crate) part_ids: Vec<usize>,
    pub(crate) options_id: Option<usize>,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rescue {
    pub(crate) body_id: Option<usize>,
    pub(crate) rescue_body_ids: Vec<usize>,
    pub(crate) else_id: Option<usize>,

    pub(crate) else_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RescueBody {
    pub(crate) exc_list_id: Option<usize>,
    pub(crate) exc_var_id: Option<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) assoc_l: Option<Loc>,
    pub(crate) begin_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Restarg {
    pub(crate) name: Option<String>,

    pub(crate) operator_l: Loc,
    pub(crate) name_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Retry;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Return {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SClass {
    pub(crate) expr_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) operator_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Self_;

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Shadowarg {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Splat {
    pub(crate) value_id: Option<usize>,

    pub(crate) operator_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Str {
    pub(crate) value: Bytes,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Super {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sym {
    pub(crate) name: String,

    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct True;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Undef {
    pub(crate) name_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnlessGuard {
    pub(crate) cond_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Until {
    pub(crate) cond_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UntilPost {
    pub(crate) cond_id: usize,
    pub(crate) body_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct When {
    pub(crate) pattern_ids: Vec<usize>,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct While {
    pub(crate) cond_id: usize,
    pub(crate) body_id: Option<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct WhilePost {
    pub(crate) cond_id: usize,
    pub(crate) body_id: usize,

    pub(crate) keyword_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct XHeredoc {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) heredoc_body_l: Loc,
    pub(crate) heredoc_end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Xstr {
    pub(crate) part_ids: Vec<usize>,

    pub(crate) begin_l: Loc,
    pub(crate) end_l: Loc,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Yield {
    pub(crate) arg_ids: Vec<usize>,

    pub(crate) keyword_l: Loc,
    pub(crate) begin_l: Option<Loc>,
    pub(crate) end_l: Option<Loc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ZSuper;
