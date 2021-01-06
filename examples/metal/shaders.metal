#include <metal_stdlib>

using namespace metal;

typedef struct {
	packed_float2 position;
} vertex_t;

struct ColorInOut {
    float4 position [[position]];
};

// vertex shader function
vertex ColorInOut triangle_vertex(const device vertex_t* vertex_array [[ buffer(0) ]],
                                   unsigned int vid [[ vertex_id ]])
{
    ColorInOut out;

    auto device const &v = vertex_array[vid];
    out.position = float4(v.position.x, v.position.y, 0.0, 1.0);

    return out;
}

// fragment shader function
fragment float4 triangle_fragment(ColorInOut in [[stage_in]])
{
    return float4(0.0, 0.5, 0.5, 1.0);
};