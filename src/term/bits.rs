use types::{Uint, UINT_SIZEOF};

const _TAG_PRIMARY_SIZE: uint = 2;   // bits
const _TAG_PRIMARY_MASK: Uint = 0x3;

const TAG_PRIMARY_HEADER: Uint = 0x0;
const TAG_PRIMARY_LIST: Uint = 0x1;
const TAG_PRIMARY_BOXED: Uint = 0x2;
const TAG_PRIMARY_IMMED1: Uint = 0x3;

#[inline(always)]
fn primary_tag(x: Uint) -> Uint {
  return x & _TAG_PRIMARY_MASK;
}

const _TAG_IMMED1_SIZE: uint = 4; // bits
const _TAG_IMMED1_MASK: Uint = 0xF;

const _TAG_IMMED1_PID: Uint = ((0x0_u64 << _TAG_PRIMARY_SIZE) | TAG_PRIMARY_IMMED1);
const _TAG_IMMED1_PORT: Uint = ((0x1 << _TAG_PRIMARY_SIZE) | TAG_PRIMARY_IMMED1);
const _TAG_IMMED1_IMMED2: Uint = ((0x2 << _TAG_PRIMARY_SIZE) | TAG_PRIMARY_IMMED1);
const _TAG_IMMED1_SMALL: Uint = ((0x3 << _TAG_PRIMARY_SIZE) | TAG_PRIMARY_IMMED1);

const _TAG_IMMED2_SIZE: uint = 6; // bits
const _TAG_IMMED2_MASK: Uint = 0x3F;
const _TAG_IMMED2_ATOM: Uint = ((0x0 << _TAG_IMMED1_SIZE) | _TAG_IMMED1_IMMED2);
const _TAG_IMMED2_CATCH: Uint = ((0x1 << _TAG_IMMED1_SIZE) | _TAG_IMMED1_IMMED2);
const _TAG_IMMED2_NIL: Uint = ((0x3 << _TAG_IMMED1_SIZE) | _TAG_IMMED1_IMMED2);

/*
 * HEADER representation:
 *
 *  aaaaaaaaaaaaaaaaaaaaaaaaaatttt00  arity:26, tag:4
 *
 * HEADER tags:
 *
 *  0000  ARITYVAL
 *      0001    BINARY_AGGREGATE                |
 *  001x  BIGNUM with sign bit    |
 *  0100  REF       |
 *  0101  FUN       | THINGS
 *  0110  FLONUM        |
 *      0111    EXPORT                          |
 *  1000  REFC_BINARY |   |
 *  1001  HEAP_BINARY | BINARIES  |
 *  1010  SUB_BINARY  |   |
 *      1011    Not used; see comment below
 *      1100    EXTERNAL_PID  |                 |
 *      1101    EXTERNAL_PORT | EXTERNAL THINGS |
 *      1110    EXTERNAL_REF  |                 |
 *      1111    MAP
 *
 * COMMENTS:
 *
 * - The tag is zero for arityval and non-zero for thing headers.
 * - A single bit differentiates between positive and negative bignums.
 * - If more tags are needed, the REF and and EXTERNAL_REF tags could probably
 *   be combined to one tag.
 *
 * XXX: globally replace XXX_SUBTAG with TAG_HEADER_XXX
 */
const ARITYVAL_SUBTAG: Uint = (0x0 << _TAG_PRIMARY_SIZE); // TUPLE
const BIN_MATCHSTATE_SUBTAG: Uint = (0x1 << _TAG_PRIMARY_SIZE);
const POS_BIG_SUBTAG: Uint = (0x2 << _TAG_PRIMARY_SIZE); //BIG: tags 2&3
const NEG_BIG_SUBTAG: Uint = (0x3 << _TAG_PRIMARY_SIZE); // BIG: tags 2&3
const _BIG_SIGN_BIT: Uint = (0x1 << _TAG_PRIMARY_SIZE);
const REF_SUBTAG: Uint = (0x4 << _TAG_PRIMARY_SIZE); // REF
const FUN_SUBTAG: Uint = (0x5 << _TAG_PRIMARY_SIZE); // FUN
const FLOAT_SUBTAG: Uint = (0x6 << _TAG_PRIMARY_SIZE); // FLOAT
const EXPORT_SUBTAG: Uint = (0x7 << _TAG_PRIMARY_SIZE); // FLOAT
const _BINARY_XXX_MASK: Uint = (0x3 << _TAG_PRIMARY_SIZE);
const REFC_BINARY_SUBTAG: Uint = (0x8 << _TAG_PRIMARY_SIZE); // BINARY
const HEAP_BINARY_SUBTAG: Uint = (0x9 << _TAG_PRIMARY_SIZE); // BINARY
const SUB_BINARY_SUBTAG: Uint = (0xA << _TAG_PRIMARY_SIZE); // BINARY
//   _BINARY_XXX_MASK depends on 0xB being unused
const EXTERNAL_PID_SUBTAG: Uint = (0xC << _TAG_PRIMARY_SIZE); // EXTERNAL_PID
const EXTERNAL_PORT_SUBTAG: Uint = (0xD << _TAG_PRIMARY_SIZE); // EXTERNAL_PORT
const EXTERNAL_REF_SUBTAG: Uint = (0xE << _TAG_PRIMARY_SIZE); // EXTERNAL_REF
const MAP_SUBTAG: Uint = (0xF << _TAG_PRIMARY_SIZE); // MAP

const _TAG_HEADER_ARITYVAL: Uint = (TAG_PRIMARY_HEADER|ARITYVAL_SUBTAG);
const _TAG_HEADER_FUN: Uint = (TAG_PRIMARY_HEADER|FUN_SUBTAG);
const _TAG_HEADER_POS_BIG: Uint = (TAG_PRIMARY_HEADER|POS_BIG_SUBTAG);
const _TAG_HEADER_NEG_BIG: Uint = (TAG_PRIMARY_HEADER|NEG_BIG_SUBTAG);
const _TAG_HEADER_FLOAT: Uint = (TAG_PRIMARY_HEADER|FLOAT_SUBTAG);
const _TAG_HEADER_EXPORT: Uint = (TAG_PRIMARY_HEADER|EXPORT_SUBTAG);
const _TAG_HEADER_REF: Uint = (TAG_PRIMARY_HEADER|REF_SUBTAG);
const _TAG_HEADER_REFC_BIN: Uint = (TAG_PRIMARY_HEADER|REFC_BINARY_SUBTAG);
const _TAG_HEADER_HEAP_BIN: Uint = (TAG_PRIMARY_HEADER|HEAP_BINARY_SUBTAG);
const _TAG_HEADER_SUB_BIN: Uint = (TAG_PRIMARY_HEADER|SUB_BINARY_SUBTAG);
const _TAG_HEADER_EXTERNAL_PID: Uint = (TAG_PRIMARY_HEADER|EXTERNAL_PID_SUBTAG);
const _TAG_HEADER_EXTERNAL_PORT: Uint = (TAG_PRIMARY_HEADER|EXTERNAL_PORT_SUBTAG);
const _TAG_HEADER_EXTERNAL_REF: Uint = (TAG_PRIMARY_HEADER|EXTERNAL_REF_SUBTAG);
const _TAG_HEADER_BIN_MATCHSTATE: Uint = (TAG_PRIMARY_HEADER|BIN_MATCHSTATE_SUBTAG);
const _TAG_HEADER_MAP: Uint = (TAG_PRIMARY_HEADER|MAP_SUBTAG);

const _TAG_HEADER_MASK: Uint = 0x3F;
const _HEADER_SUBTAG_MASK: Uint = 0x3C;  // 4 bits for subtag

const _HEADER_ARITY_OFFS: uint = 6; // bits

#[inline(always)]
pub fn non_value() -> Uint {
  return _make_header(!0, _TAG_HEADER_FLOAT);
}

#[inline(always)]
fn _make_header(sz: Uint, tag: Uint) -> Uint {
  return (sz << _HEADER_ARITY_OFFS) + tag;
}

//
// NIL access methods
//
/*pub const NIL: Uint = (!0 << _TAG_IMMED2_SIZE) | _TAG_IMMED2_NIL;
#[inline(always)]
pub fn is_nil(x: Eterm) -> bool {
  return x == NIL;
}
*/

const MAX_ATOM_INDEX: Uint = !(!0 << (UINT_SIZEOF*8 - _TAG_IMMED2_SIZE));

//
// atom access methods
//
#[inline(always)]
pub fn make_atom(x: uint) -> uint {
  return (x << _TAG_IMMED2_SIZE) + _TAG_IMMED2_ATOM as uint;
}
#[inline(always)]
pub fn is_atom(x: Uint) -> bool {
  return ((x) & _TAG_IMMED2_MASK) == _TAG_IMMED2_ATOM;
}
#[inline(always)]
fn _unchecked_atom_val(x: Uint) -> Uint {
  return x >> _TAG_IMMED2_SIZE;
}
//_ET_DECLARE_CHECKED(Uint, atom_val, Eterm)
//#define atom_val(x) _ET_APPLY(atom_val,(x))
