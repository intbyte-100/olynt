

macro_rules! implement_mask {
    ($t:ty) => {
        impl MaskElement<$t> for $t {

            #[inline(always)]
            fn zero() -> $t {
                0
            }

            #[inline(always)]
            fn one() -> $t {
                1
            }

            #[inline(always)]
            fn lshift(&self, size: $t) -> $t {
                self << size
            }



            #[inline(always)]
            fn bitand(&self, value: $t) -> $t {
                self & value
            }

            #[inline(always)]
            fn bitor(&self, value: $t) -> $t {
                self | value
            }

            #[inline(always)]
            fn set_mask(&mut self, value: $t) {
                *self = value;
            }

            #[inline(always)]
            fn to_bool(&self) -> bool {
                Self::zero() != *self
            }

        }
    };
}
pub trait MaskElement<T>
where
    T: MaskElement<T>,
    T: Copy,
{

    fn zero() -> T;
    fn one() -> T;
    fn lshift(&self, size: T) -> T;
    fn bitand(&self, value: T) -> T;
    fn bitor(&self, value: T) -> T;
    fn set_mask(&mut self, value: T);
    fn to_bool(&self) -> bool;

    #[inline(always)]
    fn set(&mut self, index: T, value: bool) {
        let value = match value {
            true => Self::one().lshift(index),
            false => Self::zero(),
        };

        self.set_mask(self.bitor(value))
    }
}

implement_mask!(i8);
implement_mask!(i16);
implement_mask!(i32);
implement_mask!(i64);
implement_mask!(i128);

pub struct Mask<T>
where
    T: MaskElement<T>,
    T: Copy,
{
    mask: T,
}

impl<T> Mask<T>
where
    T: MaskElement<T>,
    T: Copy,
{
    #[inline]
    pub fn set(&mut self, index: T, value: bool) {
        self.mask.set(index, value)
    }
    #[inline]
    pub fn get(&self, index: T) -> bool {
        self.mask.bitand(T::one().lshift(index)).to_bool()
    }

    pub fn as_int(&self) -> T {
        self.mask
    }

    pub fn new() -> Mask<T>{
        Self {mask: T::zero()}
    }
}
