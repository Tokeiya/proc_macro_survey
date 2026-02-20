use mermaid_writer::prelude::*;
use proc_macro2::{Span, TokenStream};
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

pub struct Visit;

impl<'a> visit::Visit<'a> for Visit {
	fn visit_item(&mut self, i: &'a Item) {
		println!("Visiting item");
		visit::visit_item(self, i)
	}

	fn visit_abi(&mut self, i: &'a Abi) {
		println!("Visiting abi");
		visit::visit_abi(self, i)
	}

	fn visit_data_struct(&mut self, i: &'a syn::DataStruct) {
		println!("Visiting data struct");
		visit::visit_data_struct(self, i)
	}

	fn visit_data_union(&mut self, i: &'a syn::DataUnion) {
		println!("Visiting data union");
		visit::visit_data_union(self, i)
	}

	fn visit_angle_bracketed_generic_arguments(&mut self, i: &'a AngleBracketedGenericArguments) {
		println!("Visiting angle bracketed generic arguments");
		visit::visit_angle_bracketed_generic_arguments(self, i)
	}

	fn visit_generic_argument(&mut self, i: &'a syn::GenericArgument) {
		println!("Visiting generic argument");
		visit::visit_generic_argument(self, i)
	}

	fn visit_generic_param(&mut self, i: &'a syn::GenericParam) {
		println!("Visiting generic param");
		visit::visit_generic_param(self, i)
	}

	fn visit_lifetime(&mut self, i: &'a syn::Lifetime) {
		println!("Visiting lifetime");
		visit::visit_lifetime(self, i)
	}

	fn visit_type_param(&mut self, i: &'a syn::TypeParam) {
		println!("Visiting type param");
		visit::visit_type_param(self, i)
	}

	fn visit_where_clause(&mut self, i: &'a syn::WhereClause) {
		println!("Visiting where clause");
		visit::visit_where_clause(self, i)
	}

	fn visit_where_predicate(&mut self, i: &'a syn::WherePredicate) {
		println!("Visiting where predicate");
		visit::visit_where_predicate(self, i)
	}

	fn visit_arm(&mut self, i: &'a Arm) {
		println!("Visiting arm");
		visit::visit_arm(self, i)
	}

	fn visit_assoc_const(&mut self, i: &'a AssocConst) {
		println!("Visiting associated constant");
		visit::visit_assoc_const(self, i)
	}

	fn visit_assoc_type(&mut self, i: &'a AssocType) {
		println!("Visiting associated type");
		visit::visit_assoc_type(self, i)
	}

	fn visit_attribute(&mut self, i: &'a syn::Attribute) {
		println!("Visiting attribute");
		visit::visit_attribute(self, i)
	}

	fn visit_attr_style(&mut self, i: &'a AttrStyle) {
		println!("Visiting attribute style");
		visit::visit_attr_style(self, i)
	}

	fn visit_bare_fn_arg(&mut self, i: &'a BareFnArg) {
		println!("Visiting bare function argument");
		visit::visit_bare_fn_arg(self, i)
	}

	fn visit_bare_variadic(&mut self, i: &'a BareVariadic) {
		println!("Visiting bare variadic");
		visit::visit_bare_variadic(self, i)
	}

	fn visit_bin_op(&mut self, i: &'a BinOp) {
		println!("Visiting binary operator");
		visit::visit_bin_op(self, i)
	}

	fn visit_block(&mut self, i: &'a Block) {
		println!("Visiting block");
		visit::visit_block(self, i)
	}

	fn visit_bound_lifetimes(&mut self, i: &'a BoundLifetimes) {
		println!("Visiting bound lifetimes");
		visit::visit_bound_lifetimes(self, i)
	}

	fn visit_captured_param(&mut self, i: &'a CapturedParam) {
		println!("Visiting captured param");
		visit::visit_captured_param(self, i)
	}

	fn visit_const_param(&mut self, i: &'a ConstParam) {
		println!("Visiting const param");
		visit::visit_const_param(self, i)
	}

	fn visit_constraint(&mut self, i: &'a Constraint) {
		println!("Visiting constraint");
		visit::visit_constraint(self, i)
	}

	fn visit_data(&mut self, i: &'a Data) {
		println!("Visiting data");
		visit::visit_data(self, i)
	}

	fn visit_derive_input(&mut self, i: &'a DeriveInput) {
		println!("Visiting derive input");
		visit::visit_derive_input(self, i)
	}

	fn visit_expr(&mut self, i: &'a Expr) {
		println!("Visiting expression");
		visit::visit_expr(self, i)
	}

	fn visit_expr_array(&mut self, i: &'a ExprArray) {
		println!("Visiting expression array");
		visit::visit_expr_array(self, i)
	}

	fn visit_expr_assign(&mut self, i: &'a ExprAssign) {
		println!("Visiting expression assignment");
		visit::visit_expr_assign(self, i)
	}

	fn visit_expr_async(&mut self, i: &'a ExprAsync) {
		println!("Visiting expression async");
		visit::visit_expr_async(self, i)
	}

	fn visit_expr_await(&mut self, i: &'a ExprAwait) {
		println!("Visiting expression await");
		visit::visit_expr_await(self, i)
	}

	fn visit_expr_binary(&mut self, i: &'a ExprBinary) {
		println!("Visiting expression binary");
		visit::visit_expr_binary(self, i)
	}

	fn visit_expr_block(&mut self, i: &'a ExprBlock) {
		println!("Visiting expression block");
		visit::visit_expr_block(self, i)
	}

	fn visit_expr_break(&mut self, i: &'a ExprBreak) {
		println!("Visiting expression break");
		visit::visit_expr_break(self, i)
	}

	fn visit_expr_call(&mut self, i: &'a ExprCall) {
		println!("Visiting expression call");
		visit::visit_expr_call(self, i)
	}

	fn visit_expr_cast(&mut self, i: &'a ExprCast) {
		println!("Visiting expression cast");
		visit::visit_expr_cast(self, i)
	}

	fn visit_expr_closure(&mut self, i: &'a ExprClosure) {
		println!("Visiting expression closure");
		visit::visit_expr_closure(self, i)
	}

	fn visit_expr_const(&mut self, i: &'a ExprConst) {
		println!("Visiting expression const");
		visit::visit_expr_const(self, i)
	}

	fn visit_expr_continue(&mut self, i: &'a ExprContinue) {
		println!("Visiting expression continue");
		visit::visit_expr_continue(self, i)
	}

	fn visit_expr_field(&mut self, i: &'a ExprField) {
		println!("Visiting expression field");
		visit::visit_expr_field(self, i)
	}

	fn visit_expr_for_loop(&mut self, i: &'a ExprForLoop) {
		println!("Visiting expression for loop");
		visit::visit_expr_for_loop(self, i)
	}

	fn visit_expr_group(&mut self, i: &'a ExprGroup) {
		println!("Visiting expression group");
		visit::visit_expr_group(self, i)
	}

	fn visit_expr_if(&mut self, i: &'a ExprIf) {
		println!("Visiting expression if");
		visit::visit_expr_if(self, i)
	}
	fn visit_data_enum(&mut self, i: &'a DataEnum) {
		println!("Visiting data enum");
		visit::visit_data_enum(self, i)
	}

	fn visit_expr_index(&mut self, i: &'a ExprIndex) {
		println!("Visiting expression index");
		visit::visit_expr_index(self, i)
	}

	fn visit_expr_infer(&mut self, i: &'a ExprInfer) {
		println!("Visiting expression infer");
		visit::visit_expr_infer(self, i)
	}

	fn visit_expr_let(&mut self, i: &'a ExprLet) {
		println!("Visiting expression let");
		visit::visit_expr_let(self, i)
	}

	fn visit_expr_lit(&mut self, i: &'a ExprLit) {
		println!("Visiting expression literal");
		visit::visit_expr_lit(self, i)
	}

	fn visit_expr_loop(&mut self, i: &'a ExprLoop) {
		println!("Visiting expression loop");
		visit::visit_expr_loop(self, i)
	}

	fn visit_expr_macro(&mut self, i: &'a ExprMacro) {
		println!("Visiting expression macro");
		visit::visit_expr_macro(self, i)
	}

	fn visit_expr_match(&mut self, i: &'a ExprMatch) {
		println!("Visiting expression match");
		visit::visit_expr_match(self, i)
	}

	fn visit_expr_method_call(&mut self, i: &'a ExprMethodCall) {
		println!("Visiting expression method call");
		visit::visit_expr_method_call(self, i)
	}

	fn visit_expr_paren(&mut self, i: &'a ExprParen) {
		println!("Visiting expression paren");
		visit::visit_expr_paren(self, i)
	}

	fn visit_expr_path(&mut self, i: &'a ExprPath) {
		println!("Visiting expression path");
		visit::visit_expr_path(self, i)
	}

	fn visit_expr_range(&mut self, i: &'a ExprRange) {
		println!("Visiting expression range");
		visit::visit_expr_range(self, i)
	}

	fn visit_expr_raw_addr(&mut self, i: &'a ExprRawAddr) {
		println!("Visiting expression raw address");
		visit::visit_expr_raw_addr(self, i)
	}

	fn visit_expr_reference(&mut self, i: &'a ExprReference) {
		println!("Visiting expression reference");
		visit::visit_expr_reference(self, i)
	}

	fn visit_expr_repeat(&mut self, i: &'a ExprRepeat) {
		println!("Visiting expression repeat");
		visit::visit_expr_repeat(self, i)
	}

	fn visit_expr_return(&mut self, i: &'a syn::ExprReturn) {
		println!("Visiting expression return");
		visit::visit_expr_return(self, i)
	}

	fn visit_expr_struct(&mut self, i: &'a syn::ExprStruct) {
		println!("Visiting expression struct");
		visit::visit_expr_struct(self, i)
	}

	fn visit_expr_try(&mut self, i: &'a syn::ExprTry) {
		println!("Visiting expression try");
		visit::visit_expr_try(self, i)
	}

	fn visit_expr_try_block(&mut self, i: &'a syn::ExprTryBlock) {
		println!("Visiting expression try block");
		visit::visit_expr_try_block(self, i)
	}

	fn visit_expr_tuple(&mut self, i: &'a syn::ExprTuple) {
		println!("Visiting expression tuple");
		visit::visit_expr_tuple(self, i)
	}

	fn visit_expr_unary(&mut self, i: &'a ExprUnary) {
		println!("Visiting expression unary");
		visit::visit_expr_unary(self, i)
	}

	fn visit_expr_unsafe(&mut self, i: &'a ExprUnsafe) {
		println!("Visiting expression unsafe");
		visit::visit_expr_unsafe(self, i)
	}

	fn visit_expr_while(&mut self, i: &'a syn::ExprWhile) {
		println!("Visiting expression while");
		visit::visit_expr_while(self, i)
	}

	fn visit_expr_yield(&mut self, i: &'a ExprYield) {
		println!("Visiting expression yield");
		visit::visit_expr_yield(self, i)
	}

	fn visit_field(&mut self, i: &'a Field) {
		println!("Visiting field");
		visit::visit_field(self, i)
	}

	fn visit_generics(&mut self, i: &'a syn::Generics) {
		println!("Visiting generics");
		visit::visit_generics(self, i)
	}

	fn visit_ident(&mut self, i: &'a syn::Ident) {
		println!("Visiting identifier");
		visit::visit_ident(self, i)
	}

	fn visit_impl_item(&mut self, i: &'a syn::ImplItem) {
		println!("Visiting impl item");
		visit::visit_impl_item(self, i)
	}

	fn visit_item_const(&mut self, i: &'a syn::ItemConst) {
		println!("Visiting item const");
		visit::visit_item_const(self, i)
	}

	fn visit_item_enum(&mut self, i: &'a syn::ItemEnum) {
		println!("Visiting item enum");
		visit::visit_item_enum(self, i)
	}

	fn visit_item_fn(&mut self, i: &'a syn::ItemFn) {
		println!("Visiting item function");
		visit::visit_item_fn(self, i)
	}

	fn visit_item_mod(&mut self, i: &'a syn::ItemMod) {
		println!("Visiting item module");
		visit::visit_item_mod(self, i)
	}

	fn visit_item_struct(&mut self, i: &'a syn::ItemStruct) {
		println!("Visiting item struct");
		visit::visit_item_struct(self, i)
	}

	fn visit_item_trait(&mut self, i: &'a syn::ItemTrait) {
		println!("Visiting item trait");
		visit::visit_item_trait(self, i)
	}

	fn visit_item_union(&mut self, i: &'a syn::ItemUnion) {
		println!("Visiting item union");
		visit::visit_item_union(self, i)
	}

	fn visit_item_use(&mut self, i: &'a syn::ItemUse) {
		println!("Visiting item use");
		visit::visit_item_use(self, i)
	}

	fn visit_field_mutability(&mut self, i: &'a FieldMutability) {
		println!("Visiting field mutability");
		visit::visit_field_mutability(self, i)
	}

	fn visit_field_pat(&mut self, i: &'a FieldPat) {
		println!("Visiting field pattern");
		visit::visit_field_pat(self, i)
	}

	fn visit_field_value(&mut self, i: &'a FieldValue) {
		println!("Visiting field value");
		visit::visit_field_value(self, i)
	}

	fn visit_foreign_item(&mut self, i: &'a syn::ForeignItem) {
		println!("Visiting foreign item");
		visit::visit_foreign_item(self, i)
	}

	fn visit_foreign_item_fn(&mut self, i: &'a syn::ForeignItemFn) {
		println!("Visiting foreign item function");
		visit::visit_foreign_item_fn(self, i)
	}

	fn visit_foreign_item_static(&mut self, i: &'a syn::ForeignItemStatic) {
		println!("Visiting foreign item static");
		visit::visit_foreign_item_static(self, i)
	}

	fn visit_foreign_item_type(&mut self, i: &'a syn::ForeignItemType) {
		println!("Visiting foreign item type");
		visit::visit_foreign_item_type(self, i)
	}

	fn visit_foreign_item_macro(&mut self, i: &'a syn::ForeignItemMacro) {
		println!("Visiting foreign item macro");
		visit::visit_foreign_item_macro(self, i)
	}

	fn visit_fields(&mut self, i: &'a Fields) {
		println!("Visiting fields");
		visit::visit_fields(self, i)
	}

	fn visit_fields_named(&mut self, i: &'a FieldsNamed) {
		println!("Visiting named fields");
		visit::visit_fields_named(self, i)
	}

	fn visit_fields_unnamed(&mut self, i: &'a syn::FieldsUnnamed) {
		println!("Visiting unnamed fields");
		visit::visit_fields_unnamed(self, i)
	}

	fn visit_file(&mut self, i: &'a File) {
		println!("Visiting file");
		visit::visit_file(self, i)
	}

	fn visit_fn_arg(&mut self, i: &'a FnArg) {
		println!("Visiting function argument");
		visit::visit_fn_arg(self, i)
	}

	fn visit_impl_item_const(&mut self, i: &'a ImplItemConst) {
		println!("Visiting impl item constant");
		visit::visit_impl_item_const(self, i)
	}

	fn visit_impl_item_fn(&mut self, i: &'a ImplItemFn) {
		println!("Visiting impl item function");
		visit::visit_impl_item_fn(self, i)
	}

	fn visit_impl_item_macro(&mut self, i: &'a ImplItemMacro) {
		println!("Visiting impl item macro");
		visit::visit_impl_item_macro(self, i)
	}

	fn visit_impl_item_type(&mut self, i: &'a ImplItemType) {
		println!("Visiting impl item type");
		visit::visit_impl_item_type(self, i)
	}

	fn visit_impl_restriction(&mut self, i: &'a ImplRestriction) {
		println!("Visiting impl restriction");
		visit::visit_impl_restriction(self, i)
	}

	fn visit_index(&mut self, i: &'a Index) {
		println!("Visiting index");
		visit::visit_index(self, i)
	}

	fn visit_item_extern_crate(&mut self, i: &'a ItemExternCrate) {
		println!("Visiting item extern crate");
		visit::visit_item_extern_crate(self, i)
	}

	fn visit_item_foreign_mod(&mut self, i: &'a ItemForeignMod) {
		println!("Visiting item foreign mod");
		visit::visit_item_foreign_mod(self, i)
	}

	fn visit_item_impl(&mut self, i: &'a ItemImpl) {
		println!("Visiting item impl");
		visit::visit_item_impl(self, i)
	}

	fn visit_item_macro(&mut self, i: &'a ItemMacro) {
		println!("Visiting item macro");
		visit::visit_item_macro(self, i)
	}

	fn visit_item_static(&mut self, i: &'a ItemStatic) {
		println!("Visiting item static");
		visit::visit_item_static(self, i)
	}

	fn visit_item_trait_alias(&mut self, i: &'a ItemTraitAlias) {
		println!("Visiting item trait alias");
		visit::visit_item_trait_alias(self, i)
	}

	fn visit_item_type(&mut self, i: &'a ItemType) {
		println!("Visiting item type");
		visit::visit_item_type(self, i)
	}

	fn visit_label(&mut self, i: &'a Label) {
		println!("Visiting label");
		visit::visit_label(self, i)
	}

	fn visit_lifetime_param(&mut self, i: &'a LifetimeParam) {
		println!("Visiting lifetime parameter");
		visit::visit_lifetime_param(self, i)
	}

	fn visit_lit(&mut self, i: &'a Lit) {
		println!("Visiting literal");
		visit::visit_lit(self, i)
	}

	fn visit_lit_bool(&mut self, i: &'a LitBool) {
		println!("Visiting boolean literal");
		visit::visit_lit_bool(self, i)
	}

	fn visit_lit_byte(&mut self, i: &'a syn::LitByte) {
		println!("Visiting byte literal");
		visit::visit_lit_byte(self, i)
	}

	fn visit_lit_byte_str(&mut self, i: &'a syn::LitByteStr) {
		println!("Visiting byte string literal");
		visit::visit_lit_byte_str(self, i)
	}

	fn visit_lit_char(&mut self, i: &'a syn::LitChar) {
		println!("Visiting char literal");
		visit::visit_lit_char(self, i)
	}

	fn visit_lit_float(&mut self, i: &'a syn::LitFloat) {
		println!("Visiting float literal");
		visit::visit_lit_float(self, i)
	}

	fn visit_lit_int(&mut self, i: &'a syn::LitInt) {
		println!("Visiting integer literal");
		visit::visit_lit_int(self, i)
	}

	fn visit_lit_str(&mut self, i: &'a syn::LitStr) {
		println!("Visiting string literal");
		visit::visit_lit_str(self, i)
	}

	fn visit_lit_cstr(&mut self, i: &'a LitCStr) {
		println!("Visiting C string literal");
		visit::visit_lit_cstr(self, i)
	}

	fn visit_macro(&mut self, i: &'a syn::Macro) {
		println!("Visiting macro");
		visit::visit_macro(self, i)
	}

	fn visit_local(&mut self, i: &'a Local) {
		println!("Visiting local variable");
		visit::visit_local(self, i)
	}

	fn visit_local_init(&mut self, i: &'a LocalInit) {
		println!("Visiting local variable initialization");
		visit::visit_local_init(self, i)
	}

	fn visit_macro_delimiter(&mut self, i: &'a MacroDelimiter) {
		println!("Visiting macro delimiter");
		visit::visit_macro_delimiter(self, i)
	}

	fn visit_member(&mut self, i: &'a Member) {
		println!("Visiting member");
		visit::visit_member(self, i)
	}

	fn visit_meta(&mut self, i: &'a Meta) {
		println!("Visiting meta");
		visit::visit_meta(self, i)
	}

	fn visit_meta_list(&mut self, i: &'a MetaList) {
		println!("Visiting meta list");
		visit::visit_meta_list(self, i)
	}

	fn visit_meta_name_value(&mut self, i: &'a MetaNameValue) {
		println!("Visiting meta name-value");
		visit::visit_meta_name_value(self, i)
	}

	fn visit_parenthesized_generic_arguments(&mut self, i: &'a ParenthesizedGenericArguments) {
		println!("Visiting parenthesized generic arguments");
		visit::visit_parenthesized_generic_arguments(self, i)
	}

	fn visit_pat(&mut self, i: &'a Pat) {
		println!("Visiting pattern");
		visit::visit_pat(self, i)
	}

	fn visit_pat_ident(&mut self, i: &'a PatIdent) {
		println!("Visiting pattern identifier");
		visit::visit_pat_ident(self, i)
	}

	fn visit_pat_or(&mut self, i: &'a PatOr) {
		println!("Visiting pattern or");
		visit::visit_pat_or(self, i)
	}

	fn visit_pat_paren(&mut self, i: &'a PatParen) {
		println!("Visiting pattern paren");
		visit::visit_pat_paren(self, i)
	}

	fn visit_pat_reference(&mut self, i: &'a PatReference) {
		println!("Visiting pattern reference");
		visit::visit_pat_reference(self, i)
	}

	fn visit_pat_rest(&mut self, i: &'a PatRest) {
		println!("Visiting pattern rest");
		visit::visit_pat_rest(self, i)
	}

	fn visit_pat_slice(&mut self, i: &'a PatSlice) {
		println!("Visiting pattern slice");
		visit::visit_pat_slice(self, i)
	}

	fn visit_pat_struct(&mut self, i: &'a PatStruct) {
		println!("Visiting pattern struct");
		visit::visit_pat_struct(self, i)
	}

	fn visit_pat_tuple(&mut self, i: &'a PatTuple) {
		println!("Visiting pattern tuple");
		visit::visit_pat_tuple(self, i)
	}

	fn visit_pat_tuple_struct(&mut self, i: &'a PatTupleStruct) {
		println!("Visiting pattern tuple struct");
		visit::visit_pat_tuple_struct(self, i)
	}

	fn visit_pat_type(&mut self, i: &'a PatType) {
		println!("Visiting pattern type");
		visit::visit_pat_type(self, i)
	}

	fn visit_path(&mut self, i: &'a syn::Path) {
		println!("Visiting path");
		visit::visit_path(self, i)
	}

	fn visit_path_arguments(&mut self, i: &'a syn::PathArguments) {
		println!("Visiting path arguments");
		visit::visit_path_arguments(self, i)
	}

	fn visit_path_segment(&mut self, i: &'a syn::PathSegment) {
		println!("Visiting path segment");
		visit::visit_path_segment(self, i)
	}

	fn visit_qself(&mut self, i: &'a syn::QSelf) {
		println!("Visiting qualified self");
		visit::visit_qself(self, i)
	}

	fn visit_range_limits(&mut self, i: &'a syn::RangeLimits) {
		println!("Visiting range limits");
		visit::visit_range_limits(self, i)
	}

	fn visit_stmt(&mut self, i: &'a syn::Stmt) {
		println!("Visiting statement");
		visit::visit_stmt(self, i)
	}

	fn visit_token_stream(&mut self, i: &'a TokenStream) {
		println!("Visiting token stream");
	}

	fn visit_pat_wild(&mut self, i: &'a PatWild) {
		println!("Visiting pattern wildcard");
		visit::visit_pat_wild(self, i)
	}

	fn visit_pointer_mutability(&mut self, i: &'a PointerMutability) {
		println!("Visiting pointer mutability");
		visit::visit_pointer_mutability(self, i)
	}

	fn visit_precise_capture(&mut self, i: &'a PreciseCapture) {
		println!("Visiting precise capture");
		visit::visit_precise_capture(self, i)
	}

	fn visit_predicate_lifetime(&mut self, i: &'a PredicateLifetime) {
		println!("Visiting predicate lifetime");
		visit::visit_predicate_lifetime(self, i)
	}

	fn visit_predicate_type(&mut self, i: &'a PredicateType) {
		println!("Visiting predicate type");
		visit::visit_predicate_type(self, i)
	}

	fn visit_receiver(&mut self, i: &'a Receiver) {
		println!("Visiting receiver");
		visit::visit_receiver(self, i)
	}

	fn visit_return_type(&mut self, i: &'a ReturnType) {
		println!("Visiting return type");
		visit::visit_return_type(self, i)
	}

	fn visit_signature(&mut self, i: &'a Signature) {
		println!("Visiting signature");
		visit::visit_signature(self, i)
	}

	fn visit_span(&mut self, i: &Span) {
		println!("Visiting span");
		visit::visit_span(self, i)
	}

	fn visit_static_mutability(&mut self, i: &'a StaticMutability) {
		println!("Visiting static mutability");
		visit::visit_static_mutability(self, i)
	}

	fn visit_stmt_macro(&mut self, i: &'a StmtMacro) {
		println!("Visiting statement macro");
		visit::visit_stmt_macro(self, i)
	}

	fn visit_trait_bound(&mut self, i: &'a TraitBound) {
		println!("Visiting trait bound");
		visit::visit_trait_bound(self, i)
	}

	fn visit_trait_bound_modifier(&mut self, i: &'a TraitBoundModifier) {
		println!("Visiting trait bound modifier");
		visit::visit_trait_bound_modifier(self, i)
	}

	fn visit_trait_item(&mut self, i: &'a syn::TraitItem) {
		println!("Visiting trait item");
		visit::visit_trait_item(self, i)
	}

	fn visit_trait_item_const(&mut self, i: &'a syn::TraitItemConst) {
		println!("Visiting trait item constant");
		visit::visit_trait_item_const(self, i)
	}

	fn visit_trait_item_macro(&mut self, i: &'a syn::TraitItemMacro) {
		println!("Visiting trait item macro");
		visit::visit_trait_item_macro(self, i)
	}

	fn visit_trait_item_fn(&mut self, i: &'a TraitItemFn) {
		println!("Visiting trait item function");
		visit::visit_trait_item_fn(self, i)
	}

	fn visit_trait_item_type(&mut self, i: &'a syn::TraitItemType) {
		println!("Visiting trait item type");
		visit::visit_trait_item_type(self, i)
	}

	fn visit_type(&mut self, i: &'a Type) {
		println!("Visiting type");
		visit::visit_type(self, i)
	}

	fn visit_type_array(&mut self, i: &'a TypeArray) {
		println!("Visiting type array");
		visit::visit_type_array(self, i)
	}

	fn visit_type_bare_fn(&mut self, i: &'a TypeBareFn) {
		println!("Visiting type bare function");
		visit::visit_type_bare_fn(self, i)
	}

	fn visit_type_group(&mut self, i: &'a TypeGroup) {
		println!("Visiting type group");
		visit::visit_type_group(self, i)
	}

	fn visit_type_impl_trait(&mut self, i: &'a syn::TypeImplTrait) {
		println!("Visiting type impl trait");
		visit::visit_type_impl_trait(self, i)
	}

	fn visit_type_infer(&mut self, i: &'a syn::TypeInfer) {
		println!("Visiting type infer");
		visit::visit_type_infer(self, i)
	}

	fn visit_type_macro(&mut self, i: &'a syn::TypeMacro) {
		println!("Visiting type macro");
		visit::visit_type_macro(self, i)
	}

	fn visit_type_param_bound(&mut self, i: &'a syn::TypeParamBound) {
		println!("Visiting type parameter bound");
		visit::visit_type_param_bound(self, i)
	}

	fn visit_type_never(&mut self, i: &'a TypeNever) {
		println!("Visiting type never");
		visit::visit_type_never(self, i)
	}

	fn visit_type_paren(&mut self, i: &'a TypeParen) {
		println!("Visiting type paren");
		visit::visit_type_paren(self, i)
	}

	fn visit_type_path(&mut self, i: &'a syn::TypePath) {
		println!("Visiting type path");
		visit::visit_type_path(self, i)
	}

	fn visit_type_ptr(&mut self, i: &'a syn::TypePtr) {
		println!("Visiting type pointer");
		visit::visit_type_ptr(self, i)
	}

	fn visit_type_reference(&mut self, i: &'a syn::TypeReference) {
		println!("Visiting type reference");
		visit::visit_type_reference(self, i)
	}

	fn visit_type_slice(&mut self, i: &'a TypeSlice) {
		println!("Visiting type slice");
		visit::visit_type_slice(self, i)
	}

	fn visit_type_trait_object(&mut self, i: &'a syn::TypeTraitObject) {
		println!("Visiting type trait object");
		visit::visit_type_trait_object(self, i)
	}

	fn visit_type_tuple(&mut self, i: &'a TypeTuple) {
		println!("Visiting type tuple");
		visit::visit_type_tuple(self, i)
	}

	fn visit_un_op(&mut self, i: &'a UnOp) {
		println!("Visiting unary operator");
		visit::visit_un_op(self, i)
	}

	fn visit_use_tree(&mut self, i: &'a syn::UseTree) {
		println!("Visiting use tree");
		visit::visit_use_tree(self, i)
	}

	fn visit_variant(&mut self, i: &'a syn::Variant) {
		println!("Visiting variant");
		visit::visit_variant(self, i)
	}

	fn visit_visibility(&mut self, i: &'a syn::Visibility) {
		println!("Visiting visibility");
		visit::visit_visibility(self, i)
	}

	fn visit_use_glob(&mut self, i: &'a UseGlob) {
		println!("Visiting use glob");
		visit::visit_use_glob(self, i)
	}

	fn visit_use_group(&mut self, i: &'a UseGroup) {
		println!("Visiting use group");
		visit::visit_use_group(self, i)
	}

	fn visit_use_name(&mut self, i: &'a syn::UseName) {
		println!("Visiting use name");
		visit::visit_use_name(self, i)
	}

	fn visit_use_rename(&mut self, i: &'a syn::UseRename) {
		println!("Visiting use rename");
		visit::visit_use_rename(self, i)
	}

	fn visit_use_path(&mut self, i: &'a UsePath) {
		println!("Visiting use path");
		visit::visit_use_path(self, i)
	}

	fn visit_variadic(&mut self, i: &'a Variadic) {
		println!("Visiting variadic");
		visit::visit_variadic(self, i)
	}

	fn visit_vis_restricted(&mut self, i: &'a VisRestricted) {
		println!("Visiting restricted visibility");
		visit::visit_vis_restricted(self, i)
	}
}
