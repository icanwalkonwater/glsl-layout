
use vec::{
    bvec2, bvec3, bvec4,
    ivec2, ivec3, ivec4,
    uvec2, uvec3, uvec4,
     vec2,  vec3,  vec4,
    dvec2, dvec3, dvec4,
};

use array::Array;

/// Matrix of 2 x 2 boolean values.
pub type bmat2x2 = Array<bvec2, [bvec2; 2]>;

/// Matrix of 2 x 3 boolean values.
pub type bmat2x3 = Array<bvec3, [bvec3; 2]>;

/// Matrix of 2 x 4 boolean values.
pub type bmat2x4 = Array<bvec4, [bvec4; 2]>;

/// Matrix of 3 x 2 boolean values.
pub type bmat3x2 = Array<bvec2, [bvec2; 3]>;

/// Matrix of 3 x 3 boolean values.
pub type bmat3x3 = Array<bvec3, [bvec3; 3]>;

/// Matrix of 3 x 4 boolean values.
pub type bmat3x4 = Array<bvec4, [bvec4; 3]>;

/// Matrix of 4 x 2 boolean values.
pub type bmat4x2 = Array<bvec2, [bvec2; 4]>;

/// Matrix of 4 x 3 boolean values.
pub type bmat4x3 = Array<bvec3, [bvec3; 4]>;

/// Matrix of 4 x 4 boolean values.
pub type bmat4x4 = Array<bvec4, [bvec4; 4]>;

/// Matrix of 2 x 2 boolean values.
pub type bmat2 = bmat2x2;

/// Matrix of 3 x 3 boolean values.
pub type bmat3 = bmat3x3;

/// Matrix of 4 x 4 boolean values.
pub type bmat4 = bmat4x4;




/// Matrix of 2 x 2 signed integer values.
pub type imat2x2 = Array<ivec2, [ivec2; 2]>;

/// Matrix of 2 x 3 signed integer values.
pub type imat2x3 = Array<ivec3, [ivec3; 2]>;

/// Matrix of 2 x 4 signed integer values.
pub type imat2x4 = Array<ivec4, [ivec4; 2]>;

/// Matrix of 3 x 2 signed integer values.
pub type imat3x2 = Array<ivec2, [ivec2; 3]>;

/// Matrix of 3 x 3 signed integer values.
pub type imat3x3 = Array<ivec3, [ivec3; 3]>;

/// Matrix of 3 x 4 signed integer values.
pub type imat3x4 = Array<ivec4, [ivec4; 3]>;

/// Matrix of 4 x 2 signed integer values.
pub type imat4x2 = Array<ivec2, [ivec2; 4]>;

/// Matrix of 4 x 3 signed integer values.
pub type imat4x3 = Array<ivec3, [ivec3; 4]>;

/// Matrix of 4 x 4 signed integer values.
pub type imat4x4 = Array<ivec4, [ivec4; 4]>;

/// Matrix of 2 x 2 signed integer values.
pub type imat2 = imat2x2;

/// Matrix of 3 x 3 signed integer values.
pub type imat3 = imat3x3;

/// Matrix of 4 x 4 signed integer values.
pub type imat4 = imat4x4;




/// Matrix of 2 x 2 unsiged integer values.
pub type umat2x2 = Array<uvec2, [uvec2; 2]>;

/// Matrix of 2 x 3 unsiged integer values.
pub type umat2x3 = Array<uvec3, [uvec3; 2]>;

/// Matrix of 2 x 4 unsiged integer values.
pub type umat2x4 = Array<uvec4, [uvec4; 2]>;

/// Matrix of 3 x 2 unsiged integer values.
pub type umat3x2 = Array<uvec2, [uvec2; 3]>;

/// Matrix of 3 x 3 unsiged integer values.
pub type umat3x3 = Array<uvec3, [uvec3; 3]>;

/// Matrix of 3 x 4 unsiged integer values.
pub type umat3x4 = Array<uvec4, [uvec4; 3]>;

/// Matrix of 4 x 2 unsiged integer values.
pub type umat4x2 = Array<uvec2, [uvec2; 4]>;

/// Matrix of 4 x 3 unsiged integer values.
pub type umat4x3 = Array<uvec3, [uvec3; 4]>;

/// Matrix of 4 x 4 unsiged integer values.
pub type umat4x4 = Array<uvec4, [uvec4; 4]>;

/// Matrix of 2 x 2 unsiged integer values.
pub type umat2 = umat2x2;

/// Matrix of 3 x 3 unsiged integer values.
pub type umat3 = umat3x3;

/// Matrix of 4 x 4 unsiged integer values.
pub type umat4 = umat4x4;




/// Matrix of 2 x 2 floating-point values.
pub type mat2x2 = Array<vec2, [vec2; 2]>;

/// Matrix of 2 x 3 floating-point values.
pub type mat2x3 = Array<vec3, [vec3; 2]>;

/// Matrix of 2 x 4 floating-point values.
pub type mat2x4 = Array<vec4, [vec4; 2]>;

/// Matrix of 3 x 2 floating-point values.
pub type mat3x2 = Array<vec2, [vec2; 3]>;

/// Matrix of 3 x 3 floating-point values.
pub type mat3x3 = Array<vec3, [vec3; 3]>;

/// Matrix of 3 x 4 floating-point values.
pub type mat3x4 = Array<vec4, [vec4; 3]>;

/// Matrix of 4 x 2 floating-point values.
pub type mat4x2 = Array<vec2, [vec2; 4]>;

/// Matrix of 4 x 3 floating-point values.
pub type mat4x3 = Array<vec3, [vec3; 4]>;

/// Matrix of 4 x 4 floating-point values.
pub type mat4x4 = Array<vec4, [vec4; 4]>;

/// Matrix of 2 x 2 floating-point values.
pub type mat2 = mat2x2;

/// Matrix of 3 x 3 floating-point values.
pub type mat3 = mat3x3;

/// Matrix of 4 x 4 floating-point values.
pub type mat4 = mat4x4;




/// Matrix of 2 x 2 double-precision floating-point values.
pub type dmat2x2 = Array<dvec2, [dvec2; 2]>;

/// Matrix of 2 x 3 double-precision floating-point values.
pub type dmat2x3 = Array<dvec3, [dvec3; 2]>;

/// Matrix of 2 x 4 double-precision floating-point values.
pub type dmat2x4 = Array<dvec4, [dvec4; 2]>;

/// Matrix of 3 x 2 double-precision floating-point values.
pub type dmat3x2 = Array<dvec2, [dvec2; 3]>;

/// Matrix of 3 x 3 double-precision floating-point values.
pub type dmat3x3 = Array<dvec3, [dvec3; 3]>;

/// Matrix of 3 x 4 double-precision floating-point values.
pub type dmat3x4 = Array<dvec4, [dvec4; 3]>;

/// Matrix of 4 x 2 double-precision floating-point values.
pub type dmat4x2 = Array<dvec2, [dvec2; 4]>;

/// Matrix of 4 x 3 double-precision floating-point values.
pub type dmat4x3 = Array<dvec3, [dvec3; 4]>;

/// Matrix of 4 x 4 double-precision floating-point values.
pub type dmat4x4 = Array<dvec4, [dvec4; 4]>;

/// Matrix of 2 x 2 double-precision floating-point values.
pub type dmat2 = dmat2x2;

/// Matrix of 3 x 3 double-precision floating-point values.
pub type dmat3 = dmat3x3;

/// Matrix of 4 x 4 double-precision floating-point values.
pub type dmat4 = dmat4x4;
