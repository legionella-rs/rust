use crate::abi::call::{ArgType, FnType, };
use crate::abi::{HasDataLayout, LayoutOf, TyLayout, TyLayoutMethods};

// X X X: We use globals and no function params, for a variety of reasons, making
// this module dubious at best.

fn classify_ret_ty<'a, Ty, C>(_cx: &C, ret: &mut ArgType<'a, Ty>)
    where Ty: TyLayoutMethods<'a, C> + Copy,
          C: LayoutOf<Ty = Ty, TyLayout = TyLayout<'a, Ty>> + HasDataLayout
{
    // Technically, 16bit and smaller is supported if the correct capabilities are
    // requested.
    ret.extend_integer_width_to(32);
    ret.make_indirect();
}

fn classify_arg_ty<'a, Ty, C>(_cx: &C, arg: &mut ArgType<'a, Ty>)
    where Ty: TyLayoutMethods<'a, C> + Copy,
          C: LayoutOf<Ty = Ty, TyLayout = TyLayout<'a, Ty>> + HasDataLayout
{
    arg.extend_integer_width_to(32);
}

pub fn compute_abi_info<'a, Ty, C>(cx: &C, fty: &mut FnType<'a, Ty>)
    where Ty: TyLayoutMethods<'a, C> + Copy,
          C: LayoutOf<Ty = Ty, TyLayout = TyLayout<'a, Ty>> + HasDataLayout
{
    if !fty.ret.is_ignore() {
        classify_ret_ty(cx, &mut fty.ret);
    }

    for arg in &mut fty.args {
        if arg.is_ignore() {
            continue;
        }
        classify_arg_ty(cx, arg);
    }
}
