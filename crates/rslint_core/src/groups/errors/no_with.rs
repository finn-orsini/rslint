use crate::rule_prelude::*;
use SyntaxKind::*;

declare_lint! {
  #[derive(Default)]
  NoWith,
  errors,
  "no-with"
}


#[typetag::serde]
impl CstRule for NoWith {
  fn check_node(&self, node: &SyntaxNode, context: &mut RuleCtx) -> Option<()> {
    if node.kind() == WITH_STMT {
        let err = context.err(self.name(), "Unexpected use of 'with' statement.")
          .primary(node.trimmed_range(), "Remove this 'with' statement");
         context.add_err(err);
    }
    None
  }
}

NoStrict
rule_tests! {
  NoWith::default(),
  err: {
    "
    with (point) {
      r = Math.sqrt(x * x + y * y);
    }
    "
  },
  ok: {
    "
    foo.bar()
    ",
    "
    point.r = Math.sqrt(x * x + y * y);
    "
  }
}
