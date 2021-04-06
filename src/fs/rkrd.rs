use std::iter;

use crate::fs::{Error, Parse, ResultExt, SliceRefExt};
use crate::geom::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct Rkrd {
    frames: Vec<Frame>,
}

impl Rkrd {
    pub fn frames(&self) -> &Vec<Frame> {
        &self.frames
    }
}

impl Parse for Rkrd {
    fn parse(input: &mut &[u8]) -> Result<Rkrd, Error> {
        input
            .take::<u32>()
            .filter(|fourcc| *fourcc == u32::from_be_bytes(*b"RKRD"))?;
        input.take::<u32>().filter(|version| *version == 0)?;

        let frames = iter::from_fn(|| (!input.is_empty()).then(|| input.take()))
            .collect::<Result<_, _>>()?;

        Ok(Rkrd { frames })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub dir: Vec3,
    pub pos: Vec3,
    pub vel0: Vec3,
    pub speed1: f32,
    pub vel: Vec3,
    pub rot_vec0: Vec3,
    pub rot_vec2: Vec3,
    pub rot0: Quat,
    pub rot1: Quat,
}

impl Parse for Frame {
    fn parse(input: &mut &[u8]) -> Result<Frame, Error> {
        Ok(Frame {
            dir: input.take()?,
            pos: input.take()?,
            vel0: input.take()?,
            speed1: input.take()?,
            vel: input.take()?,
            rot_vec0: input.take()?,
            rot_vec2: input.take()?,
            rot0: input.take()?,
            rot1: input.take()?,
        })
    }
}
