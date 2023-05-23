#version 450

layout(triangles, equal_spacing, cw) in;

layout(location = 0) patch in vec3 edge_normals[3][2];

layout(location = 0) out vec3 v_normal;

layout(push_constant) uniform PushConstantData {
    mat4 view;
} pc;



vec3 normalize_or_zero(vec3 v) {
    if (length(v) <= 0) {
        return vec3(0);
    }
    return normalize(v);
}

vec3 quadratic_bezier_curve(
    vec3 c20,  vec3 c11, vec3 c02,
    float t0, float t1
) {
    return (
        +c20 *t0 *t0
        +c11 *2 *t0 *t1
        +c02 *t1 *t1
    );
}
vec3 cubic_bezier_curve(
    vec3 c30, vec3 c21, vec3 c12, vec3 c03,
    float t0, float t1
) {
    return (
        +c30 *t0 *t0 *t0
        +c21 *3 *t0 *t0 *t1
        +c12 *3 *t0 *t1 *t1
        +c03 *t1 *t1 *t1
    );
}

vec3 curve_normal(
    vec3 c30, vec3 c21, vec3 c12, vec3 c03,
    vec3 n0, vec3 n1,
    float t0, float t1
) {
    vec3 vv = n0*t0 +n1*t1;
    vec3 dp_dt = quadratic_bezier_curve(c21-c30, c12-c21, c03-c12, t0, t1);
    vec3 dpn = normalize_or_zero(dp_dt);
    vec3 vn = vv -dpn*dot(vv, dpn);
    return normalize_or_zero(vn);
}

vec3 cubic_bezier_triangle(
    vec3 c300, vec3 c210, vec3 c120,
    vec3 c030, vec3 c021, vec3 c012,
    vec3 c003, vec3 c102, vec3 c201,
    vec3 c111,
    float t0, float t1, float t2
) {
    return (
        +c300 *t0 *t0 *t0
        +c210 *t0 *t0 *t1 *3
        +c120 *t0 *t1 *t1 *3
        +c030 *t1 *t1 *t1
        +c021 *t1 *t1 *t2 *3
        +c012 *t1 *t2 *t2 *3
        +c003 *t2 *t2 *t2
        +c102 *t2 *t2 *t0 *3
        +c201 *t2 *t0 *t0 *3
        +c111 *t0 *t1 *t2 *6
    );
}

vec3 cubic_bezier_triangle_normal(
    vec3 c300, vec3 c210, vec3 c120,
    vec3 c030, vec3 c021, vec3 c012,
    vec3 c003, vec3 c102, vec3 c201,
    vec3 c111,
    float t0, float t1, float t2
) {
    /* TODO: */
    return vec3(0);
}


void pn_triangle(
    vec3 p0, vec3 p1, vec3 p2,
    vec3 n0, vec3 n1, vec3 n2,
    float t0, float t1, float t2,
    out vec3 point, out vec3 normal
) {
    vec3 c300 = p0;
    vec3 c210 = p0 +((p1-p0) -n0*dot(n0, p1-p0)) /3;
    vec3 c120 = p1 +((p0-p1) -n1*dot(n1, p0-p1)) /3;
    vec3 c030 = p1;
    vec3 c021 = p1 +((p2-p1) -n1*dot(n1, p2-p1)) /3;
    vec3 c012 = p2 +((p1-p2) -n2*dot(n2, p1-p2)) /3;
    vec3 c003 = p2;
    vec3 c102 = p2 +((p0-p2) -n2*dot(n2, p0-p2)) /3;
    vec3 c201 = p0 +((p2-p0) -n0*dot(n0, p2-p0)) /3;
    vec3 ve = (+c210 +c120 +c021 +c012 +c102 +c201) /6;
    vec3 vv = (+c300 +c030 +c003) /3;
    vec3 c111 = ve +(ve-vv)/2;
    point = cubic_bezier_triangle(
        c300, c210, c120, c030, c021, c012, c003, c102, c201, c111,
        t0, t1, t2
    );
    normal = cubic_bezier_triangle_normal(
        c300, c210, c120, c030, c021, c012, c003, c102, c201, c111,
        t0, t1, t2
    );
}

void curves_defined_triangle(
    vec3 c300, vec3 c210, vec3 c120,
    vec3 c030, vec3 c021, vec3 c012,
    vec3 c003, vec3 c102, vec3 c201,
    float t0, float t1, float t2,
    out vec3 point, out vec3 normal
) {
    vec3 n0 = normalize_or_zero(cross(c300-c201, c210-c300));
    vec3 n1 = normalize_or_zero(cross(c030-c120, c021-c030));
    vec3 n2 = normalize_or_zero(cross(c003-c012, c102-c003));
    if (t0+t1 <= 0) {
        point = c003;
        normal = n2;
        return;
    }
    if (t1+t2 <= 0) {
        point = c300;
        normal = n0;
        return;
    }
    if (t2+t0 <= 0) {
        point = c030;
        normal = n1;
        return;
    }
    vec3 p01 = cubic_bezier_curve(c300, c210, c120, c030, t0/(t0+t1), t1/(t0+t1));
    vec3 p12 = cubic_bezier_curve(c030, c021, c012, c003, t1/(t1+t2), t2/(t1+t2));
    vec3 p20 = cubic_bezier_curve(c003, c102, c201, c300, t2/(t2+t0), t0/(t2+t0));
    vec3 n01 = curve_normal(c300, c210, c120, c030, n0, n1, t0/(t0+t1), t1/(t0+t1));
    vec3 n12 = curve_normal(c030, c021, c012, c003, n1, n2, t1/(t1+t2), t2/(t1+t2));
    vec3 n20 = curve_normal(c003, c102, c201, c300, n2, n0, t2/(t2+t0), t0/(t2+t0));
    float s = 1.0 / (+(t0*t1 +t1*t2 +t2*t0) -3*t0*t1*t2);
    float t01 = (1-t2)*t0*t1 *s;
    float t12 = (1-t0)*t1*t2 *s;
    float t20 = (1-t1)*t2*t0 *s;
    pn_triangle(
        p01, p12, p20, n01, n12, n20,
        t01, t12, t20,
        point, normal
    );
}




vec3 quartic_bezier_triangle_normal(
    vec3 c400, vec3 c040, vec3 c004,
    vec3 c310, vec3 c220, vec3 c130,
    vec3 c031, vec3 c022, vec3 c013,
    vec3 c103, vec3 c202, vec3 c301,
    vec3 c211, vec3 c121, vec3 c112,
    float t0, float t1, float t2
) {
    vec3 dp_dt0 = normalize(
        +(c400-c301) *t0 *t0 *t0
        +(c130-c031) *t1 *t1 *t1
        +(c103-c004) *t2 *t2 *t2
        +3*(c310-c211) *t0 *t0 *t1
        +3*(c220-c121) *t0 *t1 *t1
        +3*(c121-c022) *t1 *t1 *t2
        +3*(c112-c013) *t1 *t2 *t2
        +3*(c301-c202) *t2 *t2 *t0
        +3*(c202-c103) *t2 *t0 *t0
        +6*(c211-c112) *t0 *t1 *t2
    );
    vec3 dp_dt1 = normalize(
        +(c310-c400) *t0 *t0 *t0
        +(c040-c130) *t1 *t1 *t1
        +(c013-c103) *t2 *t2 *t2
        +3*(c220-c310) *t0 *t0 *t1
        +3*(c130-c220) *t0 *t1 *t1
        +3*(c031-c121) *t1 *t1 *t2
        +3*(c022-c112) *t1 *t2 *t2
        +3*(c112-c202) *t2 *t2 *t0
        +3*(c211-c301) *t2 *t0 *t0
        +6*(c121-c211) *t0 *t1 *t2
    );
    vec3 dp_dt2 = normalize(
        +(c301-c400) *t0 *t0 *t0
        +(c031-c130) *t1 *t1 *t1
        +(c004-c103) *t2 *t2 *t2
        +3*(c211-c310) *t0 *t0 *t1
        +3*(c121-c220) *t0 *t1 *t1
        +3*(c022-c121) *t1 *t1 *t2
        +3*(c013-c112) *t1 *t2 *t2
        +3*(c103-c202) *t2 *t2 *t0
        +3*(c202-c301) *t2 *t0 *t0
        +6*(c112-c211) *t0 *t1 *t2
    );
    vec3 vn01 = -cross(dp_dt0, dp_dt1);
    vec3 vn12 = -cross(dp_dt1, dp_dt2);
    vec3 vn20 = -cross(dp_dt2, dp_dt0);
    vec3 vn = vn01;
    vn = mix(vn, vn12, bvec3(length(vn) < length(vn12)));
    vn = mix(vn, vn20, bvec3(length(vn) < length(vn20)));
    if (length(vn) < 0.001) {
        vec3 vnf01 = -cross(c040-c400, c004-c400);
        vec3 vnf12 = -cross(c004-c040, c400-c040);
        vec3 vnf20 = -cross(c400-c004, c040-c004);
        vn = vnf01;
        vn = mix(vn, vnf12, bvec3(length(vn) < length(vnf12)));
        vn = mix(vn, vnf20, bvec3(length(vn) < length(vnf20)));
    }
    return normalize(vn);
}



void main(void) {
    vec3 v0 = gl_in[0].gl_Position.xyz;
    vec3 v1 = gl_in[1].gl_Position.xyz;
    vec3 v2 = gl_in[2].gl_Position.xyz;

    float r0 = gl_TessCoord.x;
    float r1 = gl_TessCoord.y;
    float r2 = gl_TessCoord.z;
    float rsum = r0 + r1 + r2;

    vec3 position;
    vec3 normal;

    vec3 c300 = v0;
    vec3 c210 = v0 +((v1-v0) -edge_normals[0][0]*dot(edge_normals[0][0], v1-v0)) /3;
    vec3 c120 = v1 +((v0-v1) -edge_normals[0][1]*dot(edge_normals[0][1], v0-v1)) /3;
    vec3 c030 = v1;
    vec3 c021 = v1 +((v2-v1) -edge_normals[1][0]*dot(edge_normals[1][0], v2-v1)) /3;
    vec3 c012 = v2 +((v1-v2) -edge_normals[1][1]*dot(edge_normals[1][1], v1-v2)) /3;
    vec3 c003 = v2;
    vec3 c102 = v2 +((v0-v2) -edge_normals[2][0]*dot(edge_normals[2][0], v0-v2)) /3;
    vec3 c201 = v0 +((v2-v0) -edge_normals[2][1]*dot(edge_normals[2][1], v2-v0)) /3;

    curves_defined_triangle(
        c300, c210, c120,
        c030, c021, c012,
        c003, c102, c201,
        r0/rsum, r1/rsum, r2/rsum,
        position, normal
    );

    vec4 v_position = pc.view * vec4(position, 1.0);
    gl_Position = v_position / v_position.w;

    v_normal = mat3(pc.view) * normal;
}
