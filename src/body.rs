// Rigid body that can collide with each other

use *;

extern crate sfml;

use std::cmp::*;

use self::sfml::system::Vector2f;

#[derive(Copy, Clone, PartialEq)]
pub enum IdealBody {
    Nothing,
    Point(f32)
}

#[derive(Copy, Clone, PartialEq)]
pub enum Body {
    Nothing,
    Point(Vector2f, f32)
}

impl IdealBody {
    pub fn to_body(&self, position : Vector2f, rotation : f32)
            -> Body {
        match self {
            &IdealBody::Nothing    => Body::Nothing,
            &IdealBody::Point(r)  => Body::Point(position, r)
        }
    }
}

impl Body {
    pub fn collide(&self, body : Body) -> bool {
        collide(*self, body)
    }
}

pub fn collide(body1: Body, body2: Body) -> bool {
    match (body1, body2) {
        (Body::Nothing, _)
            => false,
        (Body::Point(_,_), Body::Point(_,_))
            => collide_point(body1, body2),
        _   => collide(body2, body1)
    }
}

fn collide_point(body1 : Body, body2 : Body) -> bool {
    match (body1, body2) {
        (Body::Point(p1, r1), Body::Point(p2, r2))
          => distance(p1, p2) < r1 + r2,
        _ => panic!("Something magical happend.")
    }
}
