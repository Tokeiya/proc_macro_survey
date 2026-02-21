use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::visit::{self};
use syn::{
	Abi, AngleBracketedGenericArguments, Arm, AssocConst, AssocType, AttrStyle, BareFnArg,
	BareVariadic, BinOp, Block, BoundLifetimes, CapturedParam, ConstParam, Constraint, Data,
	DataEnum, DeriveInput, Expr, ExprArray, ExprAssign, ExprAsync, ExprAwait, ExprBinary,
	ExprBlock, ExprBreak, ExprCall, ExprCast, ExprClosure, ExprConst, ExprContinue, ExprField,
	ExprForLoop, ExprGroup, ExprIf, ExprIndex, ExprInfer, ExprLet, ExprLit, ExprLoop, ExprMacro,
	ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprRawAddr, ExprReference,
	ExprRepeat, ExprUnary, ExprUnsafe, ExprYield, Field, FieldMutability, FieldPat, FieldValue,
	Fields, FieldsNamed, File, FnArg, ImplItemConst, ImplItemFn, ImplItemMacro, ImplItemType,
	ImplRestriction, Index, Item, ItemExternCrate, ItemForeignMod, ItemImpl, ItemMacro, ItemStatic,
	ItemTraitAlias, ItemType, Label, LifetimeParam, Lit, LitBool, LitCStr, Local, LocalInit,
	MacroDelimiter, Member, Meta, MetaList, MetaNameValue, ParenthesizedGenericArguments, Pat,
	PatIdent, PatOr, PatParen, PatReference, PatRest, PatSlice, PatStruct, PatTuple,
	PatTupleStruct, PatType, PatWild, PointerMutability, PreciseCapture, PredicateLifetime,
	PredicateType, Receiver, ReturnType, Signature, StaticMutability, StmtMacro, TraitBound,
	TraitBoundModifier, TraitItemFn, Type, TypeArray, TypeBareFn, TypeGroup, TypeNever, TypeParen,
	TypeSlice, TypeTuple, UnOp, UseGlob, UseGroup, UsePath, Variadic, VisRestricted,
};

fn to_string(token: &impl ToTokens) -> String {
	let quoted = quote! {#token};
	quoted
		.to_string()
		.replace("\"", "\\\"")
		.replace("#", "#35;")
}

pub struct Visit {
	depth: usize,
}

impl Default for Visit {
	fn default() -> Self {
		Self { depth: 0 }
	}
}

impl Visit {
	fn print(&self, value: &str) {
		print!("{:02}:{:indent$}", self.depth, "", indent = self.depth * 2);
		println!("{value}");
	}

	fn enter(&mut self) {
		self.depth += 1;
	}

	fn exit(&mut self) {
		self.depth -= 1;
	}
}

impl<'a> visit::Visit<'a> for Visit {
	fn visit_abi(&mut self, i: &'a Abi) {
		self.enter();
		self.print("Visiting abi");
		visit::visit_abi(self, i);
		self.exit();
	}

	fn visit_angle_bracketed_generic_arguments(&mut self, i: &'a AngleBracketedGenericArguments) {
		self.enter();
		self.print("Visiting angle bracketed generic arguments");
		visit::visit_angle_bracketed_generic_arguments(self, i);
		self.exit();
	}

	fn visit_arm(&mut self, i: &'a Arm) {
		self.enter();
		self.print("Visiting arm");
		visit::visit_arm(self, i);
		self.exit();
	}
	fn visit_assoc_const(&mut self, i: &'a AssocConst) {
		self.enter();
		self.print("Visiting associated constant");
		visit::visit_assoc_const(self, i);
		self.exit();
	}

	fn visit_assoc_type(&mut self, i: &'a AssocType) {
		self.enter();
		self.print("Visiting associated type");
		visit::visit_assoc_type(self, i);
		self.exit();
	}

	fn visit_attr_style(&mut self, i: &'a AttrStyle) {
		self.enter();
		self.print("Visiting attribute style");
		visit::visit_attr_style(self, i);
		self.exit();
	}

	fn visit_attribute(&mut self, i: &'a syn::Attribute) {
		self.enter();
		self.print("Visiting attribute");
		visit::visit_attribute(self, i);
		self.exit();
	}

	fn visit_bare_fn_arg(&mut self, i: &'a BareFnArg) {
		self.enter();
		self.print("Visiting bare function argument");
		visit::visit_bare_fn_arg(self, i);
		self.exit();
	}

	fn visit_bare_variadic(&mut self, i: &'a BareVariadic) {
		self.enter();
		self.print("Visiting bare variadic");
		visit::visit_bare_variadic(self, i);
		self.exit();
	}

	fn visit_bin_op(&mut self, i: &'a BinOp) {
		self.enter();
		self.print("Visiting binary operator");
		visit::visit_bin_op(self, i);
		self.exit();
	}

	fn visit_block(&mut self, i: &'a Block) {
		self.enter();
		self.print("Visiting block");
		visit::visit_block(self, i);
		self.exit();
	}

	fn visit_bound_lifetimes(&mut self, i: &'a BoundLifetimes) {
		self.enter();
		self.print("Visiting bound lifetimes");
		visit::visit_bound_lifetimes(self, i);
		self.exit();
	}

	fn visit_captured_param(&mut self, i: &'a CapturedParam) {
		self.enter();
		self.print("Visiting captured param");
		visit::visit_captured_param(self, i);
		self.exit();
	}

	fn visit_const_param(&mut self, i: &'a ConstParam) {
		self.enter();
		self.print("Visiting const param");
		visit::visit_const_param(self, i);
		self.exit();
	}

	fn visit_constraint(&mut self, i: &'a Constraint) {
		self.enter();
		self.print("Visiting constraint");
		visit::visit_constraint(self, i);
		self.exit();
	}

	fn visit_data(&mut self, i: &'a Data) {
		self.enter();
		self.print("Visiting data");
		visit::visit_data(self, i);
		self.exit();
	}

	fn visit_data_enum(&mut self, i: &'a DataEnum) {
		self.enter();
		self.print("Visiting data enum");
		visit::visit_data_enum(self, i);
		self.exit();
	}

	fn visit_data_struct(&mut self, i: &'a syn::DataStruct) {
		self.enter();
		self.print("Visiting data struct");
		visit::visit_data_struct(self, i);
		self.exit();
	}

	fn visit_data_union(&mut self, i: &'a syn::DataUnion) {
		self.enter();
		self.print("Visiting data union");
		visit::visit_data_union(self, i);
		self.exit();
	}

	fn visit_derive_input(&mut self, i: &'a DeriveInput) {
		self.enter();
		self.print("Visiting derive input");
		visit::visit_derive_input(self, i);
		self.exit();
	}

	fn visit_expr(&mut self, i: &'a Expr) {
		self.enter();
		self.print("Visiting expression");
		visit::visit_expr(self, i);
		self.exit();
	}

	fn visit_expr_array(&mut self, i: &'a ExprArray) {
		self.enter();
		self.print("Visiting expression array");
		visit::visit_expr_array(self, i);
		self.exit();
	}

	fn visit_expr_assign(&mut self, i: &'a ExprAssign) {
		self.enter();
		self.print("Visiting expression assignment");
		visit::visit_expr_assign(self, i);
		self.exit();
	}

	fn visit_expr_async(&mut self, i: &'a ExprAsync) {
		self.enter();
		self.print("Visiting expression async");
		visit::visit_expr_async(self, i);
		self.exit();
	}

	fn visit_expr_await(&mut self, i: &'a ExprAwait) {
		self.enter();
		self.print("Visiting expression await");
		visit::visit_expr_await(self, i);
		self.exit();
	}

	fn visit_expr_binary(&mut self, i: &'a ExprBinary) {
		self.enter();
		self.print("Visiting expression binary");
		visit::visit_expr_binary(self, i);
		self.exit();
	}

	fn visit_expr_block(&mut self, i: &'a ExprBlock) {
		self.enter();
		self.print("Visiting expression block");
		visit::visit_expr_block(self, i);
		self.exit();
	}

	fn visit_expr_break(&mut self, i: &'a ExprBreak) {
		self.enter();
		self.print("Visiting expression break");
		visit::visit_expr_break(self, i);
		self.exit();
	}

	fn visit_expr_call(&mut self, i: &'a ExprCall) {
		self.enter();
		self.print("Visiting expression call");
		visit::visit_expr_call(self, i);
		self.exit();
	}

	fn visit_expr_cast(&mut self, i: &'a ExprCast) {
		self.enter();
		self.print("Visiting expression cast");
		visit::visit_expr_cast(self, i);
		self.exit();
	}

	fn visit_expr_closure(&mut self, i: &'a ExprClosure) {
		self.enter();
		self.print("Visiting expression closure");
		visit::visit_expr_closure(self, i);
		self.exit();
	}

	fn visit_expr_const(&mut self, i: &'a ExprConst) {
		self.enter();
		self.print("Visiting expression const");
		visit::visit_expr_const(self, i);
		self.exit();
	}

	fn visit_expr_continue(&mut self, i: &'a ExprContinue) {
		self.enter();
		self.print("Visiting expression continue");
		visit::visit_expr_continue(self, i);
		self.exit();
	}

	fn visit_expr_field(&mut self, i: &'a ExprField) {
		self.enter();
		self.print("Visiting expression field");
		visit::visit_expr_field(self, i);
		self.exit();
	}

	fn visit_expr_for_loop(&mut self, i: &'a ExprForLoop) {
		self.enter();
		self.print("Visiting expression for loop");
		visit::visit_expr_for_loop(self, i);
		self.exit();
	}

	fn visit_expr_group(&mut self, i: &'a ExprGroup) {
		self.enter();
		self.print("Visiting expression group");
		visit::visit_expr_group(self, i);
		self.exit();
	}

	fn visit_expr_if(&mut self, i: &'a ExprIf) {
		self.enter();
		self.print("Visiting expression if");
		visit::visit_expr_if(self, i);
		self.exit();
	}

	fn visit_expr_index(&mut self, i: &'a ExprIndex) {
		self.enter();
		self.print("Visiting expression index");
		visit::visit_expr_index(self, i);
		self.exit();
	}

	fn visit_expr_infer(&mut self, i: &'a ExprInfer) {
		self.enter();
		self.print("Visiting expression infer");
		visit::visit_expr_infer(self, i);
		self.exit();
	}

	fn visit_expr_let(&mut self, i: &'a ExprLet) {
		self.enter();
		self.print("Visiting expression let");
		visit::visit_expr_let(self, i);
		self.exit();
	}

	fn visit_expr_lit(&mut self, i: &'a ExprLit) {
		self.enter();
		self.print("Visiting expression literal");
		visit::visit_expr_lit(self, i);
		self.exit();
	}

	fn visit_expr_loop(&mut self, i: &'a ExprLoop) {
		self.enter();
		self.print("Visiting expression loop");
		visit::visit_expr_loop(self, i);
		self.exit();
	}

	fn visit_expr_macro(&mut self, i: &'a ExprMacro) {
		self.enter();
		self.print("Visiting expression macro");
		visit::visit_expr_macro(self, i);
		self.exit();
	}
	fn visit_expr_match(&mut self, i: &'a ExprMatch) {
		self.enter();
		self.print("Visiting expression match");
		visit::visit_expr_match(self, i);
		self.exit();
	}

	fn visit_expr_method_call(&mut self, i: &'a ExprMethodCall) {
		self.enter();
		self.print("Visiting expression method call");
		visit::visit_expr_method_call(self, i);
		self.exit();
	}

	fn visit_expr_paren(&mut self, i: &'a ExprParen) {
		self.enter();
		self.print("Visiting expression paren");
		visit::visit_expr_paren(self, i);
		self.exit();
	}

	fn visit_expr_path(&mut self, i: &'a ExprPath) {
		self.enter();
		self.print("Visiting expression path");
		visit::visit_expr_path(self, i);
		self.exit();
	}

	fn visit_expr_range(&mut self, i: &'a ExprRange) {
		self.enter();
		self.print("Visiting expression range");
		visit::visit_expr_range(self, i);
		self.exit();
	}

	fn visit_expr_raw_addr(&mut self, i: &'a ExprRawAddr) {
		self.enter();
		self.print("Visiting expression raw address");
		visit::visit_expr_raw_addr(self, i);
		self.exit();
	}

	fn visit_expr_reference(&mut self, i: &'a ExprReference) {
		self.enter();
		self.print("Visiting expression reference");
		visit::visit_expr_reference(self, i);
		self.exit();
	}

	fn visit_expr_repeat(&mut self, i: &'a ExprRepeat) {
		self.enter();
		self.print("Visiting expression repeat");
		visit::visit_expr_repeat(self, i);
		self.exit();
	}

	fn visit_expr_return(&mut self, i: &'a syn::ExprReturn) {
		self.enter();
		self.print("Visiting expression return");
		visit::visit_expr_return(self, i);
		self.exit();
	}

	fn visit_expr_struct(&mut self, i: &'a syn::ExprStruct) {
		self.enter();
		self.print("Visiting expression struct");
		visit::visit_expr_struct(self, i);
		self.exit();
	}

	fn visit_expr_try(&mut self, i: &'a syn::ExprTry) {
		self.enter();
		self.print("Visiting expression try");
		visit::visit_expr_try(self, i);
		self.exit();
	}

	fn visit_expr_try_block(&mut self, i: &'a syn::ExprTryBlock) {
		self.enter();
		self.print("Visiting expression try block");
		visit::visit_expr_try_block(self, i);
		self.exit();
	}

	fn visit_expr_tuple(&mut self, i: &'a syn::ExprTuple) {
		self.enter();
		self.print("Visiting expression tuple");
		visit::visit_expr_tuple(self, i);
		self.exit();
	}

	fn visit_expr_unary(&mut self, i: &'a ExprUnary) {
		self.enter();
		self.print("Visiting expression unary");
		visit::visit_expr_unary(self, i);
		self.exit();
	}

	fn visit_expr_unsafe(&mut self, i: &'a ExprUnsafe) {
		self.enter();
		self.print("Visiting expression unsafe");
		visit::visit_expr_unsafe(self, i);
		self.exit();
	}

	fn visit_expr_while(&mut self, i: &'a syn::ExprWhile) {
		self.enter();
		self.print("Visiting expression while");
		visit::visit_expr_while(self, i);
		self.exit();
	}

	fn visit_expr_yield(&mut self, i: &'a ExprYield) {
		self.enter();
		self.print("Visiting expression yield");
		visit::visit_expr_yield(self, i);
		self.exit();
	}

	fn visit_field(&mut self, i: &'a Field) {
		self.enter();
		self.print("Visiting field");
		visit::visit_field(self, i);
		self.exit();
	}

	fn visit_field_mutability(&mut self, i: &'a FieldMutability) {
		self.enter();
		self.print("Visiting field mutability");
		visit::visit_field_mutability(self, i);
		self.exit();
	}

	fn visit_field_pat(&mut self, i: &'a FieldPat) {
		self.enter();
		self.print("Visiting field pattern");
		visit::visit_field_pat(self, i);
		self.exit();
	}

	fn visit_field_value(&mut self, i: &'a FieldValue) {
		self.enter();
		self.print("Visiting field value");
		visit::visit_field_value(self, i);
		self.exit();
	}

	fn visit_fields(&mut self, i: &'a Fields) {
		self.enter();
		self.print("Visiting fields");
		visit::visit_fields(self, i);
		self.exit();
	}

	fn visit_fields_named(&mut self, i: &'a FieldsNamed) {
		self.enter();
		self.print("Visiting named fields");
		visit::visit_fields_named(self, i);
		self.exit();
	}

	fn visit_fields_unnamed(&mut self, i: &'a syn::FieldsUnnamed) {
		self.enter();
		self.print("Visiting unnamed fields");
		visit::visit_fields_unnamed(self, i);
		self.exit();
	}

	fn visit_file(&mut self, i: &'a File) {
		self.enter();
		self.print("Visiting file");
		visit::visit_file(self, i);
		self.exit();
	}

	fn visit_fn_arg(&mut self, i: &'a FnArg) {
		self.enter();
		self.print("Visiting function argument");
		visit::visit_fn_arg(self, i);
		self.exit();
	}

	fn visit_foreign_item(&mut self, i: &'a syn::ForeignItem) {
		self.enter();
		self.print("Visiting foreign item");
		visit::visit_foreign_item(self, i);
		self.exit();
	}

	fn visit_foreign_item_fn(&mut self, i: &'a syn::ForeignItemFn) {
		self.enter();
		self.print("Visiting foreign item function");
		visit::visit_foreign_item_fn(self, i);
		self.exit();
	}

	fn visit_foreign_item_macro(&mut self, i: &'a syn::ForeignItemMacro) {
		self.enter();
		self.print("Visiting foreign item macro");
		visit::visit_foreign_item_macro(self, i);
		self.exit();
	}

	fn visit_foreign_item_static(&mut self, i: &'a syn::ForeignItemStatic) {
		self.enter();
		self.print("Visiting foreign item static");
		visit::visit_foreign_item_static(self, i);
		self.exit();
	}

	fn visit_foreign_item_type(&mut self, i: &'a syn::ForeignItemType) {
		self.enter();
		self.print("Visiting foreign item type");
		visit::visit_foreign_item_type(self, i);
		self.exit();
	}

	fn visit_generic_argument(&mut self, i: &'a syn::GenericArgument) {
		self.enter();
		self.print("Visiting generic argument");
		visit::visit_generic_argument(self, i);
		self.exit();
	}

	fn visit_generic_param(&mut self, i: &'a syn::GenericParam) {
		self.enter();
		self.print("Visiting generic parameter");
		visit::visit_generic_param(self, i);
		self.exit();
	}

	fn visit_generics(&mut self, i: &'a syn::Generics) {
		self.enter();
		self.print("Visiting generics");
		visit::visit_generics(self, i);
		self.exit();
	}

	fn visit_ident(&mut self, i: &'a syn::Ident) {
		self.enter();
		self.print("Visiting identifier");
		visit::visit_ident(self, i);
		self.exit();
	}

	fn visit_impl_item(&mut self, i: &'a syn::ImplItem) {
		self.enter();
		self.print("Visiting impl item");
		visit::visit_impl_item(self, i);
		self.exit();
	}

	fn visit_impl_item_const(&mut self, i: &'a ImplItemConst) {
		self.enter();
		self.print("Visiting impl item constant");
		visit::visit_impl_item_const(self, i);
		self.exit();
	}

	fn visit_impl_item_fn(&mut self, i: &'a ImplItemFn) {
		self.enter();
		self.print("Visiting impl item function");
		visit::visit_impl_item_fn(self, i);
		self.exit();
	}

	fn visit_impl_item_macro(&mut self, i: &'a ImplItemMacro) {
		self.enter();
		self.print("Visiting impl item macro");
		visit::visit_impl_item_macro(self, i);
		self.exit();
	}

	fn visit_impl_item_type(&mut self, i: &'a ImplItemType) {
		self.enter();
		self.print("Visiting impl item type");
		visit::visit_impl_item_type(self, i);
		self.exit();
	}

	fn visit_impl_restriction(&mut self, i: &'a ImplRestriction) {
		self.enter();
		self.print("Visiting impl restriction");
		visit::visit_impl_restriction(self, i);
		self.exit();
	}

	fn visit_index(&mut self, i: &'a Index) {
		self.enter();
		self.print("Visiting index");
		visit::visit_index(self, i);
		self.exit();
	}

	fn visit_item(&mut self, i: &'a Item) {
		self.enter();
		self.print("Visiting item");
		visit::visit_item(self, i);
		self.exit();
	}

	fn visit_item_const(&mut self, i: &'a syn::ItemConst) {
		self.enter();
		self.print("Visiting item constant");
		visit::visit_item_const(self, i);
		self.exit();
	}

	fn visit_item_enum(&mut self, i: &'a syn::ItemEnum) {
		self.enter();
		self.print("Visiting item enum");
		visit::visit_item_enum(self, i);
		self.exit();
	}

	fn visit_item_extern_crate(&mut self, i: &'a ItemExternCrate) {
		self.enter();
		self.print("Visiting item extern crate");
		visit::visit_item_extern_crate(self, i);
		self.exit();
	}

	fn visit_item_fn(&mut self, i: &'a syn::ItemFn) {
		self.enter();
		self.print("Visiting item function");
		visit::visit_item_fn(self, i);
		self.exit();
	}

	fn visit_item_foreign_mod(&mut self, i: &'a ItemForeignMod) {
		self.enter();
		self.print("Visiting item foreign module");
		visit::visit_item_foreign_mod(self, i);
		self.exit();
	}

	fn visit_item_impl(&mut self, i: &'a ItemImpl) {
		self.enter();
		self.print("Visiting item impl");
		visit::visit_item_impl(self, i);
		self.exit();
	}

	fn visit_item_macro(&mut self, i: &'a ItemMacro) {
		self.enter();
		self.print("Visiting item macro");
		visit::visit_item_macro(self, i);
		self.exit();
	}

	fn visit_item_mod(&mut self, i: &'a syn::ItemMod) {
		self.enter();
		self.print("Visiting item module");
		visit::visit_item_mod(self, i);
		self.exit();
	}

	fn visit_item_static(&mut self, i: &'a ItemStatic) {
		self.enter();
		self.print("Visiting item static");
		visit::visit_item_static(self, i);
		self.exit();
	}

	fn visit_item_struct(&mut self, i: &'a syn::ItemStruct) {
		self.enter();
		self.print("Visiting item struct");
		visit::visit_item_struct(self, i);
		self.exit();
	}

	fn visit_item_trait(&mut self, i: &'a syn::ItemTrait) {
		self.enter();
		self.print("Visiting item trait");
		visit::visit_item_trait(self, i);
		self.exit();
	}

	fn visit_item_trait_alias(&mut self, i: &'a ItemTraitAlias) {
		self.enter();
		self.print("Visiting item trait alias");
		visit::visit_item_trait_alias(self, i);
		self.exit();
	}

	fn visit_item_type(&mut self, i: &'a ItemType) {
		self.enter();
		self.print("Visiting item type");
		visit::visit_item_type(self, i);
		self.exit();
	}

	fn visit_item_union(&mut self, i: &'a syn::ItemUnion) {
		self.enter();
		self.print("Visiting item union");
		visit::visit_item_union(self, i);
		self.exit();
	}

	fn visit_item_use(&mut self, i: &'a syn::ItemUse) {
		self.enter();
		self.print("Visiting item use");
		visit::visit_item_use(self, i);
		self.exit();
	}

	fn visit_label(&mut self, i: &'a Label) {
		self.enter();
		self.print("Visiting label");
		visit::visit_label(self, i);
		self.exit();
	}

	fn visit_lifetime(&mut self, i: &'a syn::Lifetime) {
		self.enter();
		self.print("Visiting lifetime");
		visit::visit_lifetime(self, i);
		self.exit();
	}

	fn visit_lifetime_param(&mut self, i: &'a LifetimeParam) {
		self.enter();
		self.print("Visiting lifetime parameter");
		visit::visit_lifetime_param(self, i);
		self.exit();
	}

	fn visit_lit(&mut self, i: &'a Lit) {
		self.enter();
		self.print("Visiting literal");
		visit::visit_lit(self, i);
		self.exit();
	}

	fn visit_lit_bool(&mut self, i: &'a LitBool) {
		self.enter();
		self.print("Visiting boolean literal");
		visit::visit_lit_bool(self, i);
		self.exit();
	}

	fn visit_lit_byte(&mut self, i: &'a syn::LitByte) {
		self.enter();
		self.print("Visiting byte literal");
		visit::visit_lit_byte(self, i);
		self.exit();
	}

	fn visit_lit_byte_str(&mut self, i: &'a syn::LitByteStr) {
		self.enter();
		self.print("Visiting byte string literal");
		visit::visit_lit_byte_str(self, i);
		self.exit();
	}

	fn visit_lit_cstr(&mut self, i: &'a LitCStr) {
		self.enter();
		self.print("Visiting C string literal");
		visit::visit_lit_cstr(self, i);
		self.exit();
	}

	fn visit_lit_char(&mut self, i: &'a syn::LitChar) {
		self.enter();
		self.print("Visiting character literal");
		visit::visit_lit_char(self, i);
		self.exit();
	}

	fn visit_lit_float(&mut self, i: &'a syn::LitFloat) {
		self.enter();
		self.print("Visiting float literal");
		visit::visit_lit_float(self, i);
		self.exit();
	}

	fn visit_lit_int(&mut self, i: &'a syn::LitInt) {
		self.enter();
		self.print("Visiting integer literal");
		visit::visit_lit_int(self, i);
		self.exit();
	}

	fn visit_lit_str(&mut self, i: &'a syn::LitStr) {
		self.enter();
		self.print("Visiting string literal");
		visit::visit_lit_str(self, i);
		self.exit();
	}

	fn visit_local(&mut self, i: &'a Local) {
		self.enter();
		self.print("Visiting local variable");
		visit::visit_local(self, i);
		self.exit();
	}

	fn visit_local_init(&mut self, i: &'a LocalInit) {
		self.enter();
		self.print("Visiting local variable initialization");
		visit::visit_local_init(self, i);
		self.exit();
	}

	fn visit_macro(&mut self, i: &'a syn::Macro) {
		self.enter();
		self.print("Visiting macro");
		visit::visit_macro(self, i);
		self.exit();
	}

	fn visit_macro_delimiter(&mut self, i: &'a MacroDelimiter) {
		self.enter();
		self.print("Visiting macro delimiter");
		visit::visit_macro_delimiter(self, i);
		self.exit();
	}

	fn visit_member(&mut self, i: &'a Member) {
		self.enter();
		self.print("Visiting member");
		visit::visit_member(self, i);
		self.exit();
	}

	fn visit_meta(&mut self, i: &'a Meta) {
		self.enter();
		self.print("Visiting meta");
		visit::visit_meta(self, i);
		self.exit();
	}

	fn visit_meta_list(&mut self, i: &'a MetaList) {
		self.enter();
		self.print("Visiting meta list");
		visit::visit_meta_list(self, i);
		self.exit();
	}

	fn visit_meta_name_value(&mut self, i: &'a MetaNameValue) {
		self.enter();
		self.print("Visiting meta name value");
		visit::visit_meta_name_value(self, i);
		self.exit();
	}

	fn visit_parenthesized_generic_arguments(&mut self, i: &'a ParenthesizedGenericArguments) {
		self.enter();
		self.print("Visiting parenthesized generic arguments");
		visit::visit_parenthesized_generic_arguments(self, i);
		self.exit();
	}

	fn visit_pat(&mut self, i: &'a Pat) {
		self.enter();
		self.print("Visiting pattern");
		visit::visit_pat(self, i);
		self.exit();
	}

	fn visit_pat_ident(&mut self, i: &'a PatIdent) {
		self.enter();
		self.print("Visiting pattern identifier");
		visit::visit_pat_ident(self, i);
		self.exit();
	}

	fn visit_pat_or(&mut self, i: &'a PatOr) {
		self.enter();
		self.print("Visiting pattern or");
		visit::visit_pat_or(self, i);
		self.exit();
	}

	fn visit_pat_paren(&mut self, i: &'a PatParen) {
		self.enter();
		self.print("Visiting pattern paren");
		visit::visit_pat_paren(self, i);
		self.exit();
	}

	fn visit_pat_reference(&mut self, i: &'a PatReference) {
		self.enter();
		self.print("Visiting pattern reference");
		visit::visit_pat_reference(self, i);
		self.exit();
	}

	fn visit_pat_rest(&mut self, i: &'a PatRest) {
		self.enter();
		self.print("Visiting pattern rest");
		visit::visit_pat_rest(self, i);
		self.exit();
	}

	fn visit_pat_slice(&mut self, i: &'a PatSlice) {
		self.enter();
		self.print("Visiting pattern slice");
		visit::visit_pat_slice(self, i);
		self.exit();
	}

	fn visit_pat_struct(&mut self, i: &'a PatStruct) {
		self.enter();
		self.print("Visiting pattern struct");
		visit::visit_pat_struct(self, i);
		self.exit();
	}

	fn visit_pat_tuple(&mut self, i: &'a PatTuple) {
		self.enter();
		self.print("Visiting pattern tuple");
		visit::visit_pat_tuple(self, i);
		self.exit();
	}

	fn visit_pat_tuple_struct(&mut self, i: &'a PatTupleStruct) {
		self.enter();
		self.print("Visiting pattern tuple struct");
		visit::visit_pat_tuple_struct(self, i);
		self.exit();
	}

	fn visit_pat_type(&mut self, i: &'a PatType) {
		self.enter();
		self.print("Visiting pattern type");
		visit::visit_pat_type(self, i);
		self.exit();
	}

	fn visit_pat_wild(&mut self, i: &'a PatWild) {
		self.enter();
		self.print("Visiting pattern wildcard");
		visit::visit_pat_wild(self, i);
		self.exit();
	}

	fn visit_path(&mut self, i: &'a syn::Path) {
		self.enter();
		self.print("Visiting path");
		visit::visit_path(self, i);
		self.exit();
	}

	fn visit_path_arguments(&mut self, i: &'a syn::PathArguments) {
		self.enter();
		self.print("Visiting path arguments");
		visit::visit_path_arguments(self, i);
		self.exit();
	}

	fn visit_path_segment(&mut self, i: &'a syn::PathSegment) {
		self.enter();
		self.print("Visiting path segment");
		visit::visit_path_segment(self, i);
		self.exit();
	}

	fn visit_pointer_mutability(&mut self, i: &'a PointerMutability) {
		self.enter();
		self.print("Visiting pointer mutability");
		visit::visit_pointer_mutability(self, i);
		self.exit();
	}

	fn visit_precise_capture(&mut self, i: &'a PreciseCapture) {
		self.enter();
		self.print("Visiting precise capture");
		visit::visit_precise_capture(self, i);
		self.exit();
	}

	fn visit_predicate_lifetime(&mut self, i: &'a PredicateLifetime) {
		self.enter();
		self.print("Visiting predicate lifetime");
		visit::visit_predicate_lifetime(self, i);
		self.exit();
	}

	fn visit_predicate_type(&mut self, i: &'a PredicateType) {
		self.enter();
		self.print("Visiting predicate type");
		visit::visit_predicate_type(self, i);
		self.exit();
	}

	fn visit_qself(&mut self, i: &'a syn::QSelf) {
		self.enter();
		self.print("Visiting qualified self");
		visit::visit_qself(self, i);
		self.exit();
	}

	fn visit_range_limits(&mut self, i: &'a syn::RangeLimits) {
		self.enter();
		self.print("Visiting range limits");
		visit::visit_range_limits(self, i);
		self.exit();
	}

	fn visit_receiver(&mut self, i: &'a Receiver) {
		self.enter();
		self.print("Visiting receiver");
		visit::visit_receiver(self, i);
		self.exit();
	}

	fn visit_return_type(&mut self, i: &'a ReturnType) {
		self.enter();
		self.print("Visiting return type");
		visit::visit_return_type(self, i);
		self.exit();
	}

	fn visit_signature(&mut self, i: &'a Signature) {
		self.enter();
		self.print("Visiting signature");
		visit::visit_signature(self, i);
		self.exit();
	}

	fn visit_span(&mut self, i: &Span) {
		self.enter();
		self.print("Visiting span");
		visit::visit_span(self, i);
		self.exit();
	}

	fn visit_static_mutability(&mut self, i: &'a StaticMutability) {
		self.enter();
		self.print("Visiting static mutability");
		visit::visit_static_mutability(self, i);
		self.exit();
	}

	fn visit_stmt(&mut self, i: &'a syn::Stmt) {
		self.enter();
		self.print("Visiting statement");
		visit::visit_stmt(self, i);
		self.exit();
	}

	fn visit_stmt_macro(&mut self, i: &'a StmtMacro) {
		self.enter();
		self.print("Visiting statement macro");
		visit::visit_stmt_macro(self, i);
		self.exit();
	}

	fn visit_token_stream(&mut self, i: &'a TokenStream) {
		self.enter();
		self.print(&format!("Token stream: {}", to_string(&i)));
		self.exit();
	}

	fn visit_trait_bound(&mut self, i: &'a TraitBound) {
		self.enter();
		self.print("Visiting trait bound");
		visit::visit_trait_bound(self, i);
		self.exit();
	}

	fn visit_trait_bound_modifier(&mut self, i: &'a TraitBoundModifier) {
		self.enter();
		self.print("Visiting trait bound modifier");
		visit::visit_trait_bound_modifier(self, i);
		self.exit();
	}

	fn visit_trait_item(&mut self, i: &'a syn::TraitItem) {
		self.enter();
		self.print("Visiting trait item");
		visit::visit_trait_item(self, i);
		self.exit();
	}

	fn visit_trait_item_const(&mut self, i: &'a syn::TraitItemConst) {
		self.enter();
		self.print("Visiting trait item constant");
		visit::visit_trait_item_const(self, i);
		self.exit();
	}

	fn visit_trait_item_fn(&mut self, i: &'a TraitItemFn) {
		self.enter();
		self.print("Visiting trait item function");
		visit::visit_trait_item_fn(self, i);
		self.exit();
	}

	fn visit_trait_item_macro(&mut self, i: &'a syn::TraitItemMacro) {
		self.enter();
		self.print("Visiting trait item macro");
		visit::visit_trait_item_macro(self, i);
		self.exit();
	}

	fn visit_trait_item_type(&mut self, i: &'a syn::TraitItemType) {
		self.enter();
		self.print("Visiting trait item type");
		visit::visit_trait_item_type(self, i);
		self.exit();
	}

	fn visit_type(&mut self, i: &'a Type) {
		self.enter();
		self.print("Visiting type");
		visit::visit_type(self, i);
		self.exit();
	}

	fn visit_type_array(&mut self, i: &'a TypeArray) {
		self.enter();
		self.print("Visiting type array");
		visit::visit_type_array(self, i);
		self.exit();
	}

	fn visit_type_bare_fn(&mut self, i: &'a TypeBareFn) {
		self.enter();
		self.print("Visiting type bare function");
		visit::visit_type_bare_fn(self, i);
		self.exit();
	}

	fn visit_type_group(&mut self, i: &'a TypeGroup) {
		self.enter();
		self.print("Visiting type group");
		visit::visit_type_group(self, i);
		self.exit();
	}

	fn visit_type_impl_trait(&mut self, i: &'a syn::TypeImplTrait) {
		self.enter();
		self.print("Visiting type impl trait");
		visit::visit_type_impl_trait(self, i);
		self.exit();
	}

	fn visit_type_infer(&mut self, i: &'a syn::TypeInfer) {
		self.enter();
		self.print("Visiting type infer");
		visit::visit_type_infer(self, i);
		self.exit();
	}

	fn visit_type_macro(&mut self, i: &'a syn::TypeMacro) {
		self.enter();
		self.print("Visiting type macro");
		visit::visit_type_macro(self, i);
		self.exit();
	}

	fn visit_type_never(&mut self, i: &'a TypeNever) {
		self.enter();
		self.print("Visiting type never");
		visit::visit_type_never(self, i);
		self.exit();
	}

	fn visit_type_param(&mut self, i: &'a syn::TypeParam) {
		self.enter();
		self.print("Visiting type param");
		visit::visit_type_param(self, i);
		self.exit();
	}

	fn visit_type_param_bound(&mut self, i: &'a syn::TypeParamBound) {
		self.enter();
		self.print("Visiting type parameter bound");
		visit::visit_type_param_bound(self, i);
		self.exit();
	}

	fn visit_type_paren(&mut self, i: &'a TypeParen) {
		self.enter();
		self.print("Visiting type paren");
		visit::visit_type_paren(self, i);
		self.exit();
	}

	fn visit_type_path(&mut self, i: &'a syn::TypePath) {
		self.enter();
		self.print("Visiting type path");
		visit::visit_type_path(self, i);
		self.exit();
	}

	fn visit_type_ptr(&mut self, i: &'a syn::TypePtr) {
		self.enter();
		self.print("Visiting type pointer");
		visit::visit_type_ptr(self, i);
		self.exit();
	}

	fn visit_type_reference(&mut self, i: &'a syn::TypeReference) {
		self.enter();
		self.print("Visiting type reference");
		visit::visit_type_reference(self, i);
		self.exit();
	}

	fn visit_type_slice(&mut self, i: &'a TypeSlice) {
		self.enter();
		self.print("Visiting type slice");
		visit::visit_type_slice(self, i);
		self.exit();
	}

	fn visit_type_trait_object(&mut self, i: &'a syn::TypeTraitObject) {
		self.enter();
		self.print("Visiting type trait object");
		visit::visit_type_trait_object(self, i);
		self.exit();
	}

	fn visit_type_tuple(&mut self, i: &'a TypeTuple) {
		self.enter();
		self.print("Visiting type tuple");
		visit::visit_type_tuple(self, i);
		self.exit();
	}

	fn visit_un_op(&mut self, i: &'a UnOp) {
		self.enter();
		self.print("Visiting unary operator");
		visit::visit_un_op(self, i);
		self.exit();
	}

	fn visit_use_glob(&mut self, i: &'a UseGlob) {
		self.enter();
		self.print("Visiting use glob");
		visit::visit_use_glob(self, i);
		self.exit();
	}

	fn visit_use_group(&mut self, i: &'a UseGroup) {
		self.enter();
		self.print("Visiting use group");
		visit::visit_use_group(self, i);
		self.exit();
	}

	fn visit_use_name(&mut self, i: &'a syn::UseName) {
		self.enter();
		self.print("Visiting use name");
		visit::visit_use_name(self, i);
		self.exit();
	}

	fn visit_use_path(&mut self, i: &'a UsePath) {
		self.enter();
		self.print("Visiting use path");
		visit::visit_use_path(self, i);
		self.exit();
	}

	fn visit_use_rename(&mut self, i: &'a syn::UseRename) {
		self.enter();
		self.print("Visiting use rename");
		visit::visit_use_rename(self, i);
		self.exit();
	}

	fn visit_use_tree(&mut self, i: &'a syn::UseTree) {
		self.enter();
		self.print("Visiting use tree");
		visit::visit_use_tree(self, i);
		self.exit();
	}

	fn visit_variadic(&mut self, i: &'a Variadic) {
		self.enter();
		self.print("Visiting variadic");
		visit::visit_variadic(self, i);
		self.exit();
	}

	fn visit_variant(&mut self, i: &'a syn::Variant) {
		self.enter();
		self.print("Visiting variant");
		visit::visit_variant(self, i);
		self.exit();
	}

	fn visit_vis_restricted(&mut self, i: &'a VisRestricted) {
		self.enter();
		self.print("Visiting restricted visibility");
		visit::visit_vis_restricted(self, i);
		self.exit();
	}

	fn visit_visibility(&mut self, i: &'a syn::Visibility) {
		self.enter();
		self.print("Visiting visibility");
		visit::visit_visibility(self, i);
		self.exit();
	}

	fn visit_where_clause(&mut self, i: &'a syn::WhereClause) {
		self.enter();
		self.print("Visiting where clause");
		visit::visit_where_clause(self, i);
		self.exit();
	}

	fn visit_where_predicate(&mut self, i: &'a syn::WherePredicate) {
		self.enter();
		self.print("Visiting where predicate");
		visit::visit_where_predicate(self, i);
		self.exit();
	}
}
