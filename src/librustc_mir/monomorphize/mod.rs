use rustc_index::vec::IndexVec;
use rustc_middle::{traits, mir};
use rustc_middle::ty::adjustment::CustomCoerceUnsized;
use rustc_middle::ty::{self, Ty, TyCtxt, Instance};
use rustc_middle::ty::query::Providers;
use rustc_span::DUMMY_SP;

pub mod collector;
pub mod partitioning;

pub fn provide(providers: &mut Providers<'_>) {
    providers.custom_intrinsic_mirgen = |_, _| { None };
    providers.custom_intrinsic_mir = custom_intrinsic_mir;
}

fn custom_intrinsic_mir<'tcx>(tcx: TyCtxt<'tcx>,
                              instance: Instance<'tcx>)
    -> Option<&'tcx mir::BodyAndCache<'tcx>>
{
    let mirgen = tcx.custom_intrinsic_mirgen(instance.def_id())?;

    let ty = instance.monomorphic_ty(tcx);
    let sig = ty.fn_sig(tcx);
    let sig = tcx.normalize_erasing_late_bound_regions(
        ty::ParamEnv::reveal_all(),
        &sig,
    );

    // no var arg calls, so we can skip monomorphizing extra arguments.
    assert!(!sig.c_variadic);

    let source_scope_local_data = mir::ClearCrossCrate::Clear;
    let source_scope = mir::SourceScopeData {
        span: DUMMY_SP,
        parent_scope: None,
        local_data: source_scope_local_data,
    };
    let source_info = mir::SourceInfo {
        span: DUMMY_SP,
        scope: mir::OUTERMOST_SOURCE_SCOPE,
    };

    let mut source_scopes = IndexVec::new();
    source_scopes.push(source_scope.clone());

    let ret_decl = mir::LocalDecl::new_return_place(sig.output(), DUMMY_SP);
    let mut local_decls = IndexVec::from_elem_n(ret_decl, 1);
    for &arg in sig.inputs().iter() {
        local_decls.push(mir::LocalDecl {
            mutability: mir::Mutability::Mut,
            local_info: mir::LocalInfo::Other,
            ty: arg,
            source_info,
            internal: false,
            user_ty: mir::UserTypeProjections::none(),
            is_block_tail: None,
        });
    }

    let gen = mir::Body::new(IndexVec::new(),
                             source_scopes,
                             local_decls,
                             Default::default(),
                             sig.inputs().len(),
                             Vec::new(),
                             source_scope.span,
                             Vec::new(),
                             None);
    let mut gen = mir::BodyAndCache::new(gen);

    mirgen.mirgen_simple_intrinsic(tcx, instance, &mut gen);
    gen.ensure_predecessors();

    Some(tcx.arena.alloc(gen))
}

pub fn custom_coerce_unsize_info<'tcx>(
    tcx: TyCtxt<'tcx>,
    source_ty: Ty<'tcx>,
    target_ty: Ty<'tcx>,
) -> CustomCoerceUnsized {
    let def_id = tcx.lang_items().coerce_unsized_trait().unwrap();

    let trait_ref = ty::Binder::bind(ty::TraitRef {
        def_id,
        substs: tcx.mk_substs_trait(source_ty, &[target_ty.into()]),
    });

    match tcx.codegen_fulfill_obligation((ty::ParamEnv::reveal_all(), trait_ref)) {
        Some(traits::VtableImpl(traits::VtableImplData { impl_def_id, .. })) => {
            tcx.coerce_unsized_info(impl_def_id).custom_kind.unwrap()
        }
        vtable => {
            bug!("invalid `CoerceUnsized` vtable: {:?}", vtable);
        }
    }
}
