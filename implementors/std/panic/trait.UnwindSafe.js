(function() {var implementors = {};
implementors["bytes"] = [{"text":"impl UnwindSafe for Bytes","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BytesMut","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !UnwindSafe for IoSliceMut&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for IntoIter&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, U&gt; UnwindSafe for Chain&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for Limit&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for Take&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;B&gt; UnwindSafe for Reader&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;B&gt; UnwindSafe for Writer&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["fnv"] = [{"text":"impl UnwindSafe for FnvHasher","synthetic":true,"types":[]}];
implementors["lastshot"] = [{"text":"impl&lt;T&gt; UnwindSafe for NoReceivers&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for NoSenders","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for Receiver&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for Sender&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for Sender&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["memchr"] = [{"text":"impl&lt;'a&gt; UnwindSafe for Memchr&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for Memchr2&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for Memchr3&lt;'a&gt;","synthetic":true,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl UnwindSafe for TokenStream","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LexError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Span","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Group","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Punct","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Ident","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Literal","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TokenTree","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Delimiter","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Spacing","synthetic":true,"types":[]},{"text":"impl UnwindSafe for IntoIter","synthetic":true,"types":[]}];
implementors["slab"] = [{"text":"impl&lt;T&gt; UnwindSafe for Slab&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for VacantEntry&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; UnwindSafe for Iter&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for IterMut&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; UnwindSafe for Drain&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["syn"] = [{"text":"impl UnwindSafe for Attribute","synthetic":true,"types":[]},{"text":"impl UnwindSafe for MetaList","synthetic":true,"types":[]},{"text":"impl UnwindSafe for MetaNameValue","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Field","synthetic":true,"types":[]},{"text":"impl UnwindSafe for FieldsNamed","synthetic":true,"types":[]},{"text":"impl UnwindSafe for FieldsUnnamed","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Variant","synthetic":true,"types":[]},{"text":"impl UnwindSafe for VisCrate","synthetic":true,"types":[]},{"text":"impl UnwindSafe for VisPublic","synthetic":true,"types":[]},{"text":"impl UnwindSafe for VisRestricted","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Arm","synthetic":true,"types":[]},{"text":"impl UnwindSafe for FieldValue","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Label","synthetic":true,"types":[]},{"text":"impl UnwindSafe for MethodTurbofish","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprArray","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprAssign","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprAssignOp","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprAsync","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprAwait","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprBinary","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprBlock","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprBox","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprBreak","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprCall","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprCast","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprClosure","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprContinue","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprField","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprForLoop","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprGroup","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprIf","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprIndex","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprLet","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprLit","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprLoop","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprMacro","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprMatch","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprMethodCall","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprParen","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprPath","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprRange","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprReference","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprRepeat","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprReturn","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprStruct","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprTry","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprTryBlock","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprTuple","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprUnary","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprUnsafe","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprWhile","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ExprYield","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Index","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BoundLifetimes","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ConstParam","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Generics","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LifetimeDef","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PredicateEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PredicateLifetime","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PredicateType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TraitBound","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeParam","synthetic":true,"types":[]},{"text":"impl UnwindSafe for WhereClause","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for ImplGenerics&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for Turbofish&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for TypeGenerics&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ForeignItemFn","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ForeignItemMacro","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ForeignItemStatic","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ForeignItemType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ImplItemConst","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ImplItemMacro","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ImplItemMethod","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ImplItemType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemConst","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemEnum","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemExternCrate","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemFn","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemForeignMod","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemImpl","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemMacro","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemMacro2","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemMod","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemStatic","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemStruct","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemTrait","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemTraitAlias","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemUnion","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ItemUse","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Receiver","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Signature","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TraitItemConst","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TraitItemMacro","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TraitItemMethod","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TraitItemType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UseGlob","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UseGroup","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UseName","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UsePath","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UseRename","synthetic":true,"types":[]},{"text":"impl UnwindSafe for File","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Lifetime","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LitBool","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LitByte","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LitByteStr","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LitChar","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LitFloat","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LitInt","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LitStr","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Macro","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DataEnum","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DataStruct","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DataUnion","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DeriveInput","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Block","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Local","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Abi","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BareFnArg","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeArray","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeBareFn","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeGroup","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeImplTrait","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeInfer","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeMacro","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeNever","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeParen","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypePath","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypePtr","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeReference","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeSlice","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeTraitObject","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeTuple","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Variadic","synthetic":true,"types":[]},{"text":"impl UnwindSafe for FieldPat","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatBox","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatIdent","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatLit","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatMacro","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatOr","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatPath","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatRange","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatReference","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatRest","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatSlice","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatStruct","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatTuple","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatTupleStruct","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PatWild","synthetic":true,"types":[]},{"text":"impl UnwindSafe for AngleBracketedGenericArguments","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Binding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Constraint","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ParenthesizedGenericArguments","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Path","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PathSegment","synthetic":true,"types":[]},{"text":"impl UnwindSafe for QSelf","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Error","synthetic":true,"types":[]},{"text":"impl UnwindSafe for AttrStyle","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Meta","synthetic":true,"types":[]},{"text":"impl UnwindSafe for NestedMeta","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Fields","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Visibility","synthetic":true,"types":[]},{"text":"impl UnwindSafe for GenericMethodArgument","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RangeLimits","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Expr","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Member","synthetic":true,"types":[]},{"text":"impl UnwindSafe for GenericParam","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TraitBoundModifier","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TypeParamBound","synthetic":true,"types":[]},{"text":"impl UnwindSafe for WherePredicate","synthetic":true,"types":[]},{"text":"impl UnwindSafe for FnArg","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ForeignItem","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ImplItem","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Item","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TraitItem","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UseTree","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Lit","synthetic":true,"types":[]},{"text":"impl UnwindSafe for StrStyle","synthetic":true,"types":[]},{"text":"impl UnwindSafe for MacroDelimiter","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Data","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BinOp","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UnOp","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Stmt","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ReturnType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Type","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Pat","synthetic":true,"types":[]},{"text":"impl UnwindSafe for GenericArgument","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PathArguments","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Underscore","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Abstract","synthetic":true,"types":[]},{"text":"impl UnwindSafe for As","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Async","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Auto","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Await","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Become","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Box","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Break","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Const","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Continue","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Crate","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Default","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Do","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Dyn","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Else","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Enum","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Extern","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Final","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Fn","synthetic":true,"types":[]},{"text":"impl UnwindSafe for For","synthetic":true,"types":[]},{"text":"impl UnwindSafe for If","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Impl","synthetic":true,"types":[]},{"text":"impl UnwindSafe for In","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Let","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Loop","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Macro","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Match","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Mod","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Move","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Mut","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Override","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Priv","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Pub","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Ref","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Return","synthetic":true,"types":[]},{"text":"impl UnwindSafe for SelfType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for SelfValue","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Static","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Struct","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Super","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Trait","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Try","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Type","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Typeof","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Union","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Unsafe","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Unsized","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Use","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Virtual","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Where","synthetic":true,"types":[]},{"text":"impl UnwindSafe for While","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Yield","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Add","synthetic":true,"types":[]},{"text":"impl UnwindSafe for AddEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for And","synthetic":true,"types":[]},{"text":"impl UnwindSafe for AndAnd","synthetic":true,"types":[]},{"text":"impl UnwindSafe for AndEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for At","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Bang","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Caret","synthetic":true,"types":[]},{"text":"impl UnwindSafe for CaretEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Colon","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Colon2","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Comma","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Div","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DivEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Dollar","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Dot","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Dot2","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Dot3","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DotDotEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Eq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for EqEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Ge","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Gt","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Le","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Lt","synthetic":true,"types":[]},{"text":"impl UnwindSafe for MulEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Ne","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Or","synthetic":true,"types":[]},{"text":"impl UnwindSafe for OrEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for OrOr","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Pound","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Question","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RArrow","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LArrow","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Rem","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RemEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for FatArrow","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Semi","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Shl","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ShlEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Shr","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ShrEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Star","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Sub","synthetic":true,"types":[]},{"text":"impl UnwindSafe for SubEq","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Tilde","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Brace","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Bracket","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Paren","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Group","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TokenBuffer","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for Cursor&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T, P&gt; UnwindSafe for Punctuated&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, P&gt; UnwindSafe for Pairs&lt;'a, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RefUnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, P&gt; !UnwindSafe for PairsMut&lt;'a, T, P&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T, P&gt; UnwindSafe for IntoPairs&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: RefUnwindSafe + UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe + UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for IntoIter&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe + UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for Iter&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for IterMut&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T, P&gt; UnwindSafe for Pair&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for Lookahead1&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !UnwindSafe for ParseBuffer&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'c, 'a&gt; UnwindSafe for StepCursor&lt;'c, 'a&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Nothing","synthetic":true,"types":[]}];
implementors["tokio"] = [{"text":"impl UnwindSafe for DirBuilder","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for File","synthetic":true,"types":[]},{"text":"impl UnwindSafe for OpenOptions","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DirEntry","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for ReadDir","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for ReadHalf&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for WriteHalf&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, S&gt; !UnwindSafe for Seek&lt;'a, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; UnwindSafe for BufReader&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;RW&gt; UnwindSafe for BufStream&lt;RW&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;RW: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;W&gt; UnwindSafe for BufWriter&lt;W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;W: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, R, W&gt; !UnwindSafe for Copy&lt;'a, R, W&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Empty","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; UnwindSafe for Lines&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Repeat","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Sink","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; UnwindSafe for Split&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; UnwindSafe for Take&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for Builder","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for Handle","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryCurrentError","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for Runtime","synthetic":true,"types":[]},{"text":"impl UnwindSafe for JoinError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for JoinHandle&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for Barrier","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BarrierWaitResult","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized&gt; UnwindSafe for Mutex&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for MutexGuard&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryLockError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for OwnedMutexGuard&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Notify","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Semaphore","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !UnwindSafe for SemaphorePermit&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for OwnedSemaphorePermit","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized&gt; UnwindSafe for RwLock&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for RwLockReadGuard&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for RwLockWriteGuard&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Sender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for SendError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RecvError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryRecvError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Sender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for UnboundedReceiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for UnboundedSender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for SendError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RecvError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ClosedError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for TrySendError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryRecvError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for SendTimeoutError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Sender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RecvError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryRecvError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Sender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; UnwindSafe for Ref&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for SendError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for DelayQueue&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for Delay","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Error","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Instant","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for Interval","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Elapsed","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Timeout&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for Expired&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Key","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()