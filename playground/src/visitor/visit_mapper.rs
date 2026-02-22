use crate::visitor::parsed_element::Element;
use proc_macro2::{Ident, Span, TokenStream};
use std::cell::RefCell;
use std::rc::Rc;
use syn::visit::Visit;
use syn::{
	Abi, AngleBracketedGenericArguments, Arm, AssocConst, AssocType, AttrStyle, Attribute,
	BareFnArg, BareVariadic, BinOp, Block, BoundLifetimes, CapturedParam, ConstParam, Constraint,
	Data, DataEnum, DataStruct, DataUnion, DeriveInput, Expr, ExprArray, ExprAssign, ExprAsync,
	ExprAwait, ExprBinary, ExprBlock, ExprBreak, ExprCall, ExprCast, ExprClosure, ExprConst,
	ExprContinue, ExprField, ExprForLoop, ExprGroup, ExprIf, ExprIndex, ExprInfer, ExprLet,
	ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange,
	ExprRawAddr, ExprReference, ExprRepeat, ExprTry, ExprUnary, ExprUnsafe, ExprWhile, ExprYield,
	Field, FieldMutability, FieldPat, FieldValue, Fields, FieldsNamed, FieldsUnnamed, File, FnArg,
	ForeignItem, ForeignItemFn, ForeignItemMacro, ForeignItemStatic, ForeignItemType,
	GenericArgument, GenericParam, Generics, ImplItem, ImplItemConst, ImplItemFn, ImplItemMacro,
	ImplItemType, ImplRestriction, Index, Item, ItemExternCrate, ItemForeignMod, ItemTraitAlias,
	ItemType, ItemUnion, Label, Lifetime, LifetimeParam, Lit, LitBool, LitByte, LitByteStr,
	LitCStr, LitChar, LitFloat, LitInt, LitStr, Local, LocalInit, Macro, MacroDelimiter, Member,
	Meta, MetaList, MetaNameValue, ParenthesizedGenericArguments, Pat, PatIdent, PatOr, PatParen,
	PatReference, PatRest, PatSlice, PatStruct, PatTuple, PatTupleStruct, PatType, PatWild, Path,
	PathArguments, PathSegment, PointerMutability, PreciseCapture, PredicateLifetime,
	PredicateType, QSelf, RangeLimits, Receiver, ReturnType, Signature, StaticMutability, Stmt,
	StmtMacro, TraitBound, TraitBoundModifier, TraitItem, TraitItemConst, TraitItemFn,
	TraitItemMacro, TraitItemType, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait,
	TypeInfer, TypeMacro, TypeNever, TypeParam, TypeParamBound, TypeParen, TypePath, TypePtr,
	TypeReference, TypeSlice, TypeTraitObject, TypeTuple, UnOp, UseGlob, UseGroup, UseName,
	UsePath, UseRename, UseTree, Variadic, Variant, VisRestricted, Visibility, WhereClause,
	WherePredicate, visit,
};

pub struct VisitMapper {
	root: Option<Rc<RefCell<Element>>>,
	stack: Vec<Rc<RefCell<Element>>>,
	verbose: bool,
}

impl Default for VisitMapper {
	fn default() -> Self {
		Self {
			root: None,
			stack: Vec::new(),
			verbose: false,
		}
	}
}

impl VisitMapper {
	pub fn new(verbose: bool) -> Self {
		Self {
			root: None,
			stack: Vec::new(),
			verbose,
		}
	}
}

impl VisitMapper {
	pub fn get_root(&mut self) -> Option<Rc<RefCell<Element>>> {
		self.root.take()
	}
}

//noinspection DuplicatedCode
impl<'a> Visit<'a> for VisitMapper {
	fn visit_abi(&mut self, i: &'a Abi) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"abi",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_abi(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_angle_bracketed_generic_arguments(&mut self, i: &'a AngleBracketedGenericArguments) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"angle_bracketed_generic_arguments",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_angle_bracketed_generic_arguments(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_arm(&mut self, i: &'a Arm) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"arm",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_arm(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_assoc_const(&mut self, i: &'a AssocConst) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"assoc_const",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_assoc_const(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_assoc_type(&mut self, i: &'a AssocType) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"assoc_type",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_assoc_type(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_attr_style(&mut self, i: &'a AttrStyle) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"attr_style".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_attr_style(self, i);
			self.stack.pop().unwrap();
		}
	}

	fn visit_attribute(&mut self, i: &'a Attribute) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"attribute",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_attribute(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_bare_fn_arg(&mut self, i: &'a BareFnArg) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"bare_fn_arg",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_bare_fn_arg(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_bare_variadic(&mut self, i: &'a BareVariadic) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"bare_variadic",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_bare_variadic(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_bin_op(&mut self, i: &'a BinOp) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"bin_op",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_bin_op(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_block(&mut self, i: &'a Block) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"block",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_block(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_bound_lifetimes(&mut self, i: &'a BoundLifetimes) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"bound_lifetimes",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_bound_lifetimes(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_captured_param(&mut self, i: &'a CapturedParam) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"captured_param",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_captured_param(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_const_param(&mut self, i: &'a ConstParam) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"const_param",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_const_param(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_constraint(&mut self, i: &'a Constraint) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"constraint",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_constraint(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_data(&mut self, i: &'a Data) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"data".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_data(self, i);
			self.stack.pop().unwrap();
		}
	}

	fn visit_data_enum(&mut self, i: &'a DataEnum) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"data_enum".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_data_enum(self, i);
			self.stack.pop().unwrap();
		}
	}

	fn visit_data_struct(&mut self, i: &'a DataStruct) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"data_struct".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_data_struct(self, i);
			self.stack.pop().unwrap();
		}
	}

	fn visit_data_union(&mut self, i: &'a DataUnion) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"data_union".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_data_union(self, i);
			self.stack.pop().unwrap();
		}
	}

	fn visit_derive_input(&mut self, i: &'a DeriveInput) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"derive_input",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_derive_input(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr(&mut self, i: &'a Expr) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_array(&mut self, i: &'a ExprArray) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_array",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_array(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_assign(&mut self, i: &'a ExprAssign) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_assign",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_assign(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_async(&mut self, i: &'a ExprAsync) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_async",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_async(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_await(&mut self, i: &'a ExprAwait) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_await",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_await(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_binary(&mut self, i: &'a ExprBinary) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_binary",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_binary(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_block(&mut self, i: &'a ExprBlock) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_block",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_block(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_break(&mut self, i: &'a ExprBreak) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_break",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_break(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_call(&mut self, i: &'a ExprCall) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_call",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_call(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_cast(&mut self, i: &'a ExprCast) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_cast",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_cast(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_closure(&mut self, i: &'a ExprClosure) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_closure",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_closure(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_const(&mut self, i: &'a ExprConst) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_const",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_const(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_continue(&mut self, i: &'a ExprContinue) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_continue",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_continue(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_field(&mut self, i: &'a ExprField) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_field",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_field(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_for_loop(&mut self, i: &'a ExprForLoop) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_for_loop",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_for_loop(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_group(&mut self, i: &'a ExprGroup) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_group",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_group(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_if(&mut self, i: &'a ExprIf) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_if",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_if(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_index(&mut self, i: &'a ExprIndex) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_index",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_index(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_infer(&mut self, i: &'a ExprInfer) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_infer",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_infer(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_let(&mut self, i: &'a ExprLet) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_let",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_let(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_lit(&mut self, i: &'a ExprLit) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_lit",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_lit(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_loop(&mut self, i: &'a ExprLoop) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_loop",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_loop(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_macro(&mut self, i: &'a ExprMacro) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_macro",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_macro(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_match(&mut self, i: &'a ExprMatch) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_match",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_match(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_method_call(&mut self, i: &'a ExprMethodCall) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_method_call",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_method_call(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_paren(&mut self, i: &'a ExprParen) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_paren",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_paren(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_path(&mut self, i: &'a ExprPath) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_path",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_path(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_range(&mut self, i: &'a ExprRange) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_range",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_range(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_raw_addr(&mut self, i: &'a ExprRawAddr) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_raw_addr",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_raw_addr(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_reference(&mut self, i: &'a ExprReference) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_reference",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_reference(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_repeat(&mut self, i: &'a ExprRepeat) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_repeat",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_repeat(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_return(&mut self, i: &'a syn::ExprReturn) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_return",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_return(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_struct(&mut self, i: &'a syn::ExprStruct) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_struct",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_struct(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_try(&mut self, i: &'a ExprTry) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_try",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_try(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_try_block(&mut self, i: &'a syn::ExprTryBlock) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_try_block",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_try_block(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_tuple(&mut self, i: &'a syn::ExprTuple) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_tuple",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_tuple(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_unary(&mut self, i: &'a ExprUnary) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_unary",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_unary(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_unsafe(&mut self, i: &'a ExprUnsafe) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_unsafe",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_unsafe(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_while(&mut self, i: &'a ExprWhile) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_while",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_while(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_expr_yield(&mut self, i: &'a ExprYield) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"expr_yield",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_expr_yield(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_field(&mut self, i: &'a Field) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"field",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_field(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_field_mutability(&mut self, i: &'a FieldMutability) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"field_mutability".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_field_mutability(self, i);
			self.stack.pop().unwrap();
		}
	}

	fn visit_field_pat(&mut self, i: &'a FieldPat) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"field_pat",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_field_pat(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_field_value(&mut self, i: &'a FieldValue) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"field_value",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_field_value(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_fields(&mut self, i: &'a Fields) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"fields",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_fields(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_fields_named(&mut self, i: &'a FieldsNamed) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"fields_named",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_fields_named(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_fields_unnamed(&mut self, i: &'a FieldsUnnamed) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"fields_unnamed",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_fields_unnamed(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_file(&mut self, i: &'a File) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"file",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
			self.stack.push(elem.clone());
		}

		visit::visit_file(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_fn_arg(&mut self, i: &'a FnArg) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"fn_arg",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_fn_arg(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_foreign_item(&mut self, i: &'a ForeignItem) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"foreign_item",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_foreign_item(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_foreign_item_fn(&mut self, i: &'a ForeignItemFn) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"foreign_item_fn",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_foreign_item_fn(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_foreign_item_macro(&mut self, i: &'a ForeignItemMacro) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"foreign_item_macro",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_foreign_item_macro(self, i);
	}

	fn visit_foreign_item_static(&mut self, i: &'a ForeignItemStatic) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"foreign_item_static",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_foreign_item_static(self, i);
	}

	fn visit_foreign_item_type(&mut self, i: &'a ForeignItemType) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"foreign_item_type",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_foreign_item_type(self, i);
	}

	fn visit_generic_argument(&mut self, i: &'a GenericArgument) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"generic_argument",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_generic_argument(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_generic_param(&mut self, i: &'a GenericParam) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"generic_param",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_generic_param(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_generics(&mut self, i: &'a Generics) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"generics",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_generics(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_ident(&mut self, i: &'a Ident) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"ident",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_ident(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_impl_item(&mut self, i: &'a ImplItem) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"impl_item",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_impl_item(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_impl_item_const(&mut self, i: &'a ImplItemConst) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"impl_item_const",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_impl_item_const(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_impl_item_fn(&mut self, i: &'a ImplItemFn) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"impl_item_fn",
			i,
			self.stack.len(),
			false,
		)));

		//noinspection DuplicatedCode
		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_impl_item_fn(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_impl_item_macro(&mut self, i: &'a ImplItemMacro) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"impl_item_macro",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_impl_item_macro(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_impl_item_type(&mut self, i: &'a ImplItemType) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"impl_item_type",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_impl_item_type(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_impl_restriction(&mut self, i: &'a ImplRestriction) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"impl_restriction".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_impl_restriction(self, i);
			self.stack.pop().unwrap();
		}
	}

	fn visit_index(&mut self, i: &'a Index) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"index",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_index(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item(&mut self, i: &'a Item) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item",
			i,
			self.stack.len(),
			true,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_const(&mut self, i: &'a syn::ItemConst) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_const",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_const(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_enum(&mut self, i: &'a syn::ItemEnum) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_enum",
			i,
			self.stack.len(),
			true,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_enum(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_extern_crate(&mut self, i: &'a ItemExternCrate) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_extern_crate",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_extern_crate(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_fn(&mut self, i: &'a syn::ItemFn) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_fn",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_fn(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_foreign_mod(&mut self, i: &'a ItemForeignMod) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_foreign_mod",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_foreign_mod(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_impl(&mut self, i: &'a syn::ItemImpl) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_impl",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_impl(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_macro(&mut self, i: &'a syn::ItemMacro) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_macro",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_macro(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_mod(&mut self, i: &'a syn::ItemMod) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_mod",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_mod(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_static(&mut self, i: &'a syn::ItemStatic) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_static",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_static(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_struct(&mut self, i: &'a syn::ItemStruct) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_struct",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_struct(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_trait(&mut self, i: &'a syn::ItemTrait) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_trait",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_trait(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_trait_alias(&mut self, i: &'a ItemTraitAlias) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_trait_alias",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_trait_alias(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_type(&mut self, i: &'a ItemType) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_type",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_type(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_union(&mut self, i: &'a ItemUnion) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_union",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_union(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_item_use(&mut self, i: &'a syn::ItemUse) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"item_use",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_item_use(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_label(&mut self, i: &'a Label) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"label",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_label(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lifetime(&mut self, i: &'a Lifetime) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lifetime",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lifetime(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lifetime_param(&mut self, i: &'a LifetimeParam) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lifetime_param",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lifetime_param(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lit(&mut self, i: &'a Lit) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lit",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lit(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lit_bool(&mut self, i: &'a LitBool) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lit_bool",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lit_bool(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lit_byte(&mut self, i: &'a LitByte) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lit_byte",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lit_byte(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lit_byte_str(&mut self, i: &'a LitByteStr) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lit_byte_str",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lit_byte_str(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lit_cstr(&mut self, i: &'a LitCStr) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lit_cstr",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lit_cstr(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lit_char(&mut self, i: &'a LitChar) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lit_char",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lit_char(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lit_float(&mut self, i: &'a LitFloat) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lit_float",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lit_float(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lit_int(&mut self, i: &'a LitInt) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lit_int",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lit_int(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_lit_str(&mut self, i: &'a LitStr) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"lit_str",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_lit_str(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_local(&mut self, i: &'a Local) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"local",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_local(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_local_init(&mut self, i: &'a LocalInit) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"local_init".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_local_init(self, i);
			self.stack.pop().unwrap();
		}
	}

	fn visit_macro(&mut self, i: &'a Macro) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"macro",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_macro(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_macro_delimiter(&mut self, i: &'a MacroDelimiter) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"macro_delimiter".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_macro_delimiter(self, i);
			self.stack.pop().unwrap();
		}
	}

	fn visit_member(&mut self, i: &'a Member) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"member",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_member(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_meta(&mut self, i: &'a Meta) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"meta",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_meta(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_meta_list(&mut self, i: &'a MetaList) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"meta_list",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_meta_list(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_meta_name_value(&mut self, i: &'a MetaNameValue) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"meta_name_value",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_meta_name_value(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_parenthesized_generic_arguments(&mut self, i: &'a ParenthesizedGenericArguments) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"parenthesized_generic_arguments",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_parenthesized_generic_arguments(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat(&mut self, i: &'a Pat) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_ident(&mut self, i: &'a PatIdent) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_ident",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_ident(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_or(&mut self, i: &'a PatOr) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_or",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_or(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_paren(&mut self, i: &'a PatParen) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_paren",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_paren(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_reference(&mut self, i: &'a PatReference) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_reference",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_reference(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_rest(&mut self, i: &'a PatRest) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_rest",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_rest(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_slice(&mut self, i: &'a PatSlice) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_slice",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_slice(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_struct(&mut self, i: &'a PatStruct) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_struct",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_struct(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_tuple(&mut self, i: &'a PatTuple) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_tuple",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_tuple(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_tuple_struct(&mut self, i: &'a PatTupleStruct) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_tuple_struct",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_tuple_struct(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_pat_type(&mut self, i: &'a PatType) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_type",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_type(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_pat_wild(&mut self, i: &'a PatWild) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"pat_wild",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pat_wild(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_path(&mut self, i: &'a Path) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"path",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_path(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_path_arguments(&mut self, i: &'a PathArguments) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"path_arguments",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_path_arguments(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_path_segment(&mut self, i: &'a PathSegment) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"path_segment",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_path_segment(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_pointer_mutability(&mut self, i: &'a PointerMutability) {
		let elem = Rc::new(RefCell::new(Element::from_string(
			"pointer_mutability".to_string(),
			"N/A".to_string(),
			self.stack.len(),
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_pointer_mutability(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_precise_capture(&mut self, i: &'a PreciseCapture) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"precise_capture".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_precise_capture(self, i);
			self.stack.pop().unwrap();
		}
	}

	//noinspection DuplicatedCode
	fn visit_predicate_lifetime(&mut self, i: &'a PredicateLifetime) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"predicate_lifetime",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_predicate_lifetime(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_predicate_type(&mut self, i: &'a PredicateType) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"predicate_type",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_predicate_type(self, i);
		self.stack.pop().unwrap();
	}
	//noinspection DuplicatedCode
	fn visit_qself(&mut self, i: &'a QSelf) {
		let elem = Rc::new(RefCell::new(Element::from_string(
			"qself".to_string(),
			"N/A".to_string(),
			self.stack.len(),
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_qself(self, i);
		self.stack.pop().unwrap();
	}
	//noinspection DuplicatedCode
	fn visit_range_limits(&mut self, i: &'a RangeLimits) {
		let elem = Rc::new(RefCell::new(Element::from_string(
			"range_limits".to_string(),
			"N/A".to_string(),
			self.stack.len(),
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
			self.stack.push(elem.clone());
		}

		visit::visit_range_limits(self, i);
		self.stack.pop().unwrap();
	}
	//noinspection DuplicatedCode
	fn visit_receiver(&mut self, i: &'a Receiver) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"receiver",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_receiver(self, i);
		self.stack.pop().unwrap();
	}
	//noinspection DuplicatedCode
	fn visit_return_type(&mut self, i: &'a ReturnType) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"return_type",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_return_type(self, i);
		self.stack.pop().unwrap();
	}
	//noinspection DuplicatedCode
	fn visit_signature(&mut self, i: &'a Signature) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"signature",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_signature(self, i);
		self.stack.pop().unwrap();
	}
	//noinspection DuplicatedCode
	fn visit_span(&mut self, i: &Span) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"span".to_string(),
				format!("{:?}", i),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_span(self, i);
			self.stack.pop().unwrap();
		}
	}
	//noinspection DuplicatedCode
	fn visit_static_mutability(&mut self, i: &'a StaticMutability) {
		if self.verbose {
			let elem = Rc::new(RefCell::new(Element::from_string(
				"static_mutability".to_string(),
				"N/A".to_string(),
				self.stack.len(),
			)));

			if let Some(parent) = self.stack.last_mut() {
				parent.borrow_mut().add_child(elem.clone());
			} else {
				self.root = Some(elem.clone());
			}

			self.stack.push(elem.clone());
			visit::visit_static_mutability(self, i);
			self.stack.pop().unwrap();
		}
	}

	//noinspection DuplicatedCode
	fn visit_stmt(&mut self, i: &'a Stmt) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"stmt",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_stmt(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_stmt_macro(&mut self, i: &'a StmtMacro) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"stmt_macro",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_stmt_macro(self, i);
		self.stack.pop().unwrap();
	}
	//noinspection DuplicatedCode
	fn visit_token_stream(&mut self, i: &'a TokenStream) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"token_stream",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}
	}
	//noinspection DuplicatedCode
	fn visit_trait_bound(&mut self, i: &'a TraitBound) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"trait_bound",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_trait_bound(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_trait_bound_modifier(&mut self, i: &'a TraitBoundModifier) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"trait_bound_modifier",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_trait_bound_modifier(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_trait_item(&mut self, i: &'a TraitItem) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"trait_item",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_trait_item(self, i);
		self.stack.pop().unwrap();
	}

	//noinspection DuplicatedCode
	fn visit_trait_item_const(&mut self, i: &'a TraitItemConst) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"trait_item_const",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_trait_item_const(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_trait_item_fn(&mut self, i: &'a TraitItemFn) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"trait_item_fn",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_trait_item_fn(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_trait_item_macro(&mut self, i: &'a TraitItemMacro) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"trait_item_macro",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_trait_item_macro(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_trait_item_type(&mut self, i: &'a TraitItemType) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"trait_item_type",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_trait_item_type(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type(&mut self, i: &'a Type) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_array(&mut self, i: &'a TypeArray) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_array",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_array(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_bare_fn(&mut self, i: &'a TypeBareFn) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_bare_fn",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_bare_fn(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_group(&mut self, i: &'a TypeGroup) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_group",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_group(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_impl_trait(&mut self, i: &'a TypeImplTrait) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_impl_trait",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_impl_trait(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_infer(&mut self, i: &'a TypeInfer) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_infer",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_infer(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_macro(&mut self, i: &'a TypeMacro) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_macro",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_macro(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_never(&mut self, i: &'a TypeNever) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_never",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_never(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_param(&mut self, i: &'a TypeParam) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_param",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_param(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_param_bound(&mut self, i: &'a TypeParamBound) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_param_bound",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_param_bound(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_paren(&mut self, i: &'a TypeParen) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_paren",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_paren(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_path(&mut self, i: &'a TypePath) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_path",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_path(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_ptr(&mut self, i: &'a TypePtr) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_ptr",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_ptr(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_reference(&mut self, i: &'a TypeReference) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_reference",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_reference(self, i);
	}

	fn visit_type_slice(&mut self, i: &'a TypeSlice) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_slice",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_slice(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_trait_object(&mut self, i: &'a TypeTraitObject) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_trait_object",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_trait_object(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_type_tuple(&mut self, i: &'a TypeTuple) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"type_tuple",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_type_tuple(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_un_op(&mut self, i: &'a UnOp) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"un_op",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_un_op(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_use_glob(&mut self, i: &'a UseGlob) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"use_glob",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_use_glob(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_use_group(&mut self, i: &'a UseGroup) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"use_group",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_use_group(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_use_name(&mut self, i: &'a UseName) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"use_name",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_use_name(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_use_path(&mut self, i: &'a UsePath) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"use_path",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_use_path(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_use_rename(&mut self, i: &'a UseRename) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"use_rename",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_use_rename(self, i);
	}

	fn visit_use_tree(&mut self, i: &'a UseTree) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"use_tree",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_use_tree(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_variadic(&mut self, i: &'a Variadic) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"variadic",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_variadic(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_variant(&mut self, i: &'a Variant) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"variant",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_variant(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_vis_restricted(&mut self, i: &'a VisRestricted) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"vis_restricted",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_vis_restricted(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_visibility(&mut self, i: &'a Visibility) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"visibility",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_visibility(self, i);
	}

	fn visit_where_clause(&mut self, i: &'a WhereClause) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"where_clause",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_where_clause(self, i);
		self.stack.pop().unwrap();
	}

	fn visit_where_predicate(&mut self, i: &'a WherePredicate) {
		let elem = Rc::new(RefCell::new(Element::from_token_stream(
			"where_predicate",
			i,
			self.stack.len(),
			false,
		)));

		if let Some(parent) = self.stack.last_mut() {
			parent.borrow_mut().add_child(elem.clone());
		} else {
			self.root = Some(elem.clone());
		}

		self.stack.push(elem.clone());
		visit::visit_where_predicate(self, i);
		self.stack.pop().unwrap();
	}
}
