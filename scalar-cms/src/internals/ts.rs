pub struct AnythingElse;
impl ts_rs::TS for AnythingElse {
   type WithoutGenerics = Self;
   fn decl() -> String { unreachable!() }
   fn decl_concrete() -> String { unreachable!() }
   fn name() -> String { unreachable!() }
   fn inline() -> String { unreachable!() }
   fn inline_flattened() -> String { "{ [propName: string]: any }".to_owned() }
}