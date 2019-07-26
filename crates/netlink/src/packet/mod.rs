
mod netlink;
mod neighbour;
mod route;


pub use self::netlink::*;
pub use self::neighbour::*;
pub use self::route::*;

const NLA_ALIGNTO: usize       = 4;

#[inline]
pub const fn align(len: usize) -> usize {
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}