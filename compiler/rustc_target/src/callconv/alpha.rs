use rustc_abi::{BackendRepr, HasDataLayout, Primitive, Reg, TyAbiInterface};

use crate::callconv::{ArgAbi, ArgExtension, FnAbi, PassMode, Uniform};

fn extend_integer<Ty>(arg: &mut ArgAbi<'_, Ty>) {
    if let BackendRepr::Scalar(scalar) = arg.layout.backend_repr
        && let Primitive::Int(integer, _) = scalar.primitive()
        && integer.size().bits() == 32
        && let PassMode::Direct(attrs) = &mut arg.mode
    {
        // The OSF ABI sign-extends both signed and unsigned 32-bit argument items.
        attrs.arg_ext = ArgExtension::Sext;
        return;
    }
    arg.extend_integer_width_to(64);
}

pub(crate) fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    let ret = &mut fn_abi.ret;
    if !ret.is_ignore() && ret.layout.is_sized() {
        // Linux follows the OSF convention: every aggregate returns in memory.
        if ret.layout.is_aggregate() || ret.layout.size.bytes() > 8 {
            ret.make_indirect();
        } else {
            extend_integer(ret);
        }
    }
    for arg in &mut fn_abi.args {
        if arg.is_ignore() || !arg.layout.is_sized() {
            continue;
        }
        if arg.layout.pass_indirectly_in_non_rustic_abis(cx) {
            arg.make_indirect();
        } else if arg.layout.is_aggregate() {
            // Aggregates are by value regardless of size. GCC assigns the first
            // six argument words to registers and spills the rest onto the stack.
            arg.cast_to(Uniform::new(Reg::i64(), arg.layout.size));
        } else {
            extend_integer(arg);
        }
    }
}
