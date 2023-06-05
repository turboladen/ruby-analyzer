use std::path::PathBuf;

use ropey::Rope;
use ruby_analyzer_tbc_parser::{
    location::LocNode,
    parser::{parse, FileSource},
    Database, ScopedIndex,
};

mod class_with_items {
    use super::*;

    const CODE: &str = r#"# frozen_string_literal: true

class Foo
  @@class_array = [1, 2, 3]
  @instance_string = "things #{42}"

  def self.bar(a, b: 42, **c)
    puts a, b, cc

    module Wat; end
  end
end

class Baz < Foo
  def billy
    # todo
  end
end"#;

    fn setup() -> (Vec<LocNode>, ScopedIndex) {
        let database = Database::default();
        let file_source = FileSource::new(&database, PathBuf::new(), Rope::from_str(CODE));

        let (loc_nodes, index) = parse(&database, file_source);

        dbg!(&loc_nodes);
        assert_eq!(4, loc_nodes.len());

        dbg!(&index);
        assert_eq!(3, index.len());

        (loc_nodes, index)
    }

    // Validate root node
    #[test]
    fn test_root_class_node() {
        let (_loc_nodes, _index) = setup();

        // let root = arena.iter().next().unwrap();

        // let ruby_root = root.get();
        // assert_eq!(&ScopeGate::default(), ruby_root.scope_gate());

        // if let Properties::Class(class) = ruby_root.properties() {
        //     assert_eq!("Foo", class.name());
        // } else {
        //     panic!("wrong type")
        // }

        // assert_eq!(31..108, ruby_root.expression_l().as_range());
    }

    // // Validate class's const
    // #[test]
    // fn test_root_class_const_node() {
    //     let arena = setup();

    //     let foo_const = {
    //         let root = arena.iter().next().unwrap();
    //         let foo_const_id = root.first_child().unwrap();
    //         arena.get(foo_const_id).unwrap()
    //     };
    //     let ruby_foo_const = foo_const.get();

    //     if let Properties::Const(const_) = ruby_foo_const.properties() {
    //         assert_eq!("Foo", const_.name());
    //     } else {
    //         panic!("wrong type")
    //     }

    //     assert_eq!(
    //         &ScopeGate::new(vec![ScopeNode::Class("Foo".to_string())]),
    //         ruby_foo_const.scope_gate()
    //     );

    //     assert_eq!(37..40, ruby_foo_const.expression_l().as_range());
    // }

    // // Validate class's begin block
    // #[test]
    // fn test_class_body_node() {
    //     let arena = setup();
    //     let begin = {
    //         let root_class = arena.iter().next().unwrap();
    //         let foo_const_id = root_class.first_child().unwrap();
    //         let foo_const = arena.get(foo_const_id).unwrap();

    //         let begin_id = foo_const.next_sibling().unwrap();
    //         arena.get(begin_id).unwrap()
    //     };

    //     let ruby_begin = begin.get();

    //     if let Properties::Begin(_) = ruby_begin.properties() {
    //     } else {
    //         panic!("wrong type")
    //     }

    //     assert_eq!(
    //         &ScopeGate::new(vec![ScopeNode::Class("Foo".to_string())]),
    //         ruby_begin.scope_gate()
    //     );

    //     assert_eq!(43..104, ruby_begin.expression_l().as_range());
    // }

    // // Validate there are no more nodes at the child level under the root class.
    // #[test]
    // fn test_no_more_first_levels() {
    //     let arena = setup();
    //     let root_class = arena.iter().next().unwrap();
    //     let foo_const_id = root_class.first_child().unwrap();
    //     let foo_const = arena.get(foo_const_id).unwrap();

    //     let begin_id = foo_const.next_sibling().unwrap();
    //     let begin = arena.get(begin_id).unwrap();
    //     assert!(begin.next_sibling().is_none());
    // }
}
